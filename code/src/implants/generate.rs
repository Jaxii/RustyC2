use crate::models::HTTPListener;

pub fn generate_http_implant(
    listener: HTTPListener
) -> bool
{
    let flag: bool = false;

    println!("[+] Generating executable for HTTP listener on port {}", listener.port);

    match std::env::current_exe()
    {
        Ok(current_path) => {
            println!("[+] Current path: {}", current_path.display());
        },
        Err(_) => {
            println!("[!] Couldn't get the current path of the executable");
        }
    }

    return flag;
}