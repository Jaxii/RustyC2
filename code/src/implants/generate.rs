use lazy_static::lazy_static;
use std::path::PathBuf;

use crate::{models::HTTPListener, settings};

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new();
}

#[cfg(target_os = "windows")]
fn compile_msbuild_project(
    implant_project_path: PathBuf,
    output_directory_path: PathBuf
) -> bool
{
    use std::process::{Command, Stdio};
    
    let output_directory_path_str_result = output_directory_path.to_str();
    if output_directory_path_str_result.is_none()
    {
        return false;
    }

    match implant_project_path.as_path().to_str() {

        Some(implant_project_path_str) =>
        {
            match Command::new(&CONFIG.binaries.vcvarsall)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("amd64_x86")
            .arg("&&")
            .arg("msbuild")
            .arg(implant_project_path_str)
            .arg("/p:configuration=Release")
            .arg("/p:DebugSymbols=false")
            .arg("/p:DebugType=none")
            .arg(format!(
                "/p:OutDir={}",
                output_directory_path_str_result.unwrap()
            ))
            .status()
            {
                Ok(_) => {
                    println!("[+] Successfully compiled the implant project with msbuild");
                    return true;
                },
                Err(_) =>
                {
                    println!("[!] Failed to generate the implant");
                }
            }
        },
        None => {}
    }

    return false;
}

#[cfg(target_os = "windows")]
pub fn generate_http_implant(
    listener: HTTPListener,
    implant_project_name: &str
) -> bool
{
    use std::path::{Ancestors, Path};

    if CONFIG.binaries.vcvarsall.is_empty()
    {
        return false;
    }

    println!("[+] Generating executable for HTTP listener on port {}", listener.port);

    let current_path_result: Result<PathBuf, std::io::Error> = std::env::current_exe();
    if current_path_result.is_err()
    {
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

    if implant_project_path.is_none()
    {
        println!("[!] Couldn't find the path of the implant project");
        return false;
    }

    let output_directory_path: PathBuf = implant_project_path.unwrap().join("output");

    match std::fs::read_dir(implant_project_path.unwrap().join("implants").join("http"))
    {
        Ok(implant_projects_dir) =>
        {
            for entry_result in implant_projects_dir
            {
                match entry_result
                {
                    Ok(entry) =>
                    {
                        match entry.file_type()
                        {
                            Ok(entry_file_type) =>
                            {
                                if entry_file_type.is_dir()
                                {
                                    if entry.file_name().to_ascii_lowercase() == implant_project_name
                                    {
                                        println!("[+] Found implant project: {}", implant_project_name);
                                        match entry.path().to_str()
                                        {
                                            Some(implant_project_path) =>
                                            {
                                                println!("[+] Found path of the implant project: {}", implant_project_path);

                                                compile_msbuild_project(
                                                    entry.path(),
                                                    output_directory_path
                                                );
                                                return true;
                                            },
                                            None =>
                                            {
                                                println!("[!] Couldn't find the path of the implant project");
                                            }
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    },
                    Err(_) => {}
                }
            }

            println!("[!] Couldn't find the implant project");
        },
        Err(_) =>
        {
            println!("[!] Couldn't list the implant projects");
        }
    };

    return true;
}

#[cfg(target_os = "linux")]
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