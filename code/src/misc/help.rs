use crate::{
    implants,
    models::{EnumImplantCommands, EnumImplantTaskCommands},
};

pub fn print_help_main() {
    let help_items: Vec<(&str, &str)> = vec![
        ("exit", "Exit from the framework"),
        ("help", "Use 'help <command>' to show command info."),
        ("implants", "Manage implants"),
        ("listeners", "Manage listeners"),
    ];

    println!();
    println!("{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }

    println!();
}

pub fn print_help_implants(implant_command: Option<EnumImplantCommands>) {
    if implant_command.is_none() {
        let help_items: Vec<(EnumImplantCommands, &str)> = vec![
            (EnumImplantCommands::Back, "Return to the main menu"),
            (EnumImplantCommands::Exit, "Exit from the framework"),
            (EnumImplantCommands::Generate, "Generate implant code"),
            (EnumImplantCommands::Help, "Show this help menu"),
            (EnumImplantCommands::List, "List all the implants"),
            (
                EnumImplantCommands::Interact,
                "Interact with a specific implant",
            ),
            (EnumImplantCommands::Kill, "Kill implant"),
            (EnumImplantCommands::Remove, "Remove an implant"),
            (EnumImplantCommands::Sleep, "Change callback delay"),
        ];

        println!("\n{0: <20}{1}", "Command", "Description");
        println!("{0: <20}{1}", "-------", "-----------");

        for item in help_items {
            println!("{0: <20}{1}", item.0.to_string(), item.1);
        }
        println!();
    } else {
        match implant_command.unwrap() {
            implant_command @ EnumImplantCommands::Back => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Go back to the previous menu"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Exit => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Exit from the framework"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Generate => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} <listener_id> <implant_project_name>\n",
                        "[+] Description:\n\t",
                        "Generate code for a new implant\n",
                        "[+] Parameters:\n",
                        "\t- listener_id: Required\n",
                        "\t- implant_project_name: Required"
                    ),
                    implant_command
                );

                let mut implant_projects: Vec<String> = Vec::new();
                implants::generate::list_http_implants(&mut implant_projects);

                if implant_projects.len() != 0 {
                    println!("[+] Available implant projects:");
                    for implant_project_name in implant_projects {
                        println!("\t{}", implant_project_name)
                    }
                } else {
                    println!("[!] No implant projects available at the moment")
                }
            }
            implant_command @ EnumImplantCommands::Help => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} [command]\n",
                        "[+] Description:\n\t",
                        "Show the help menu or  usage for a specific command\n",
                        "[+] Parameters:\n",
                        "\t- command: Optional"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::List => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show all the implants"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Interact => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} <implant_id>\n",
                        "[+] Description:\n\t",
                        "Interact with a specific implant"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Kill => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} <implant_id>\n",
                        "[+] Description:\n\t",
                        "Kill one or more implants, by sending an exit command"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Remove => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} <implant_id>\n",
                        "[+] Description:\n\t",
                        "Remove one or more implants"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Sleep => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{} <num_seconds>\n",
                        "[+] Description:\n\t",
                        "Change the sleep setting of all the implants"
                    ),
                    implant_command
                );
            }
            implant_command @ EnumImplantCommands::Tasks => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Change the sleep setting of all the implants"
                    ),
                    implant_command
                );
            }
        }
    }
}

pub fn print_help_listeners() {
    let help_items: Vec<(&str, &str)> = vec![
        ("back", "Return to the main menu"),
        ("create", "Create a new listener"),
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("list", "List all the listeners"),
        ("remove", "Remove a listener"),
        ("start", "Start/resume a specific listener"),
        ("stop", "Suspend a specific listener"),
        ("update", "Change settings of a listeners"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

pub fn print_help_listeners_create() {
    let help_items: Vec<(&str, &str)> = vec![
        ("back", "Return to the previous menu"),
        ("create", "Create a new listener"),
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("options", "Show options"),
        ("set", "Change listener settings"),
    ];

    println!();
    println!("{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }

    println!();
}

pub fn print_help_implants_interaction(implant_command: Option<EnumImplantTaskCommands>) {
    if implant_command.is_none() {
        let help_items: Vec<(EnumImplantTaskCommands, &str)> = vec![
            (EnumImplantTaskCommands::Back, "Return to the previous menu"),
            (EnumImplantTaskCommands::Exit, "Exit from the framework"),
            (
                EnumImplantTaskCommands::Hostname,
                "Show the name of the host",
            ),
            (EnumImplantTaskCommands::Info, "Show info about the implant"),
            (
                EnumImplantTaskCommands::ListFiles,
                "Show files/directories in the current path",
            ),
            (
                EnumImplantTaskCommands::Addresses,
                "Show the IP address of the host",
            ),
            (EnumImplantTaskCommands::Pwd, "Show the current path"),
            (EnumImplantTaskCommands::Tasks, "Manage tasks"),
            (
                EnumImplantTaskCommands::Whoami,
                "Print the current username",
            ),
        ];

        println!();
        println!("{0: <20}{1}", "Command", "Description");
        println!("{0: <20}{1}", "-------", "-----------");

        for item in help_items {
            println!("{0: <20}{1}", item.0, item.1);
        }

        println!();
    } else {
        match implant_command.unwrap() {
            EnumImplantTaskCommands::Addresses => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show the IP addresses of the victim host"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Back => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Go back to the previous menu"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Exit => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Exit from the framework"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Hostname => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show the name of the victim host"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Info => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show information about the implant and the victim host"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::ListFiles => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "List the files and directories in the current path"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Whoami => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show the username of the current user account"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Pwd => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n",
                        "[+] Description:\n\t",
                        "Show the path of the current working directory"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::InjectLocal => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n <shellcode_path>",
                        "[+] Description:\n\t",
                        "Inject shellcode in the local process"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::InjectRemote => {
                println!(
                    concat!(
                        "[+] Usage:\n",
                        "\t{}\n <PID> <shellcode_path>",
                        "[+] Description:\n\t",
                        "Inject shellcode in the remote process"
                    ),
                    EnumImplantTaskCommands::Addresses
                );
            }
            EnumImplantTaskCommands::Tasks => todo!(),
        }
    }
}

pub fn print_help_usage(command: &str) {
    match command {
        "back" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tback\n",
                "[+] Description:\n\t",
                "It allows you to go back to the previous menu"
            ));
        }
        "exit" => {
            println!(concat!(
                "[+] Usage:\n",
                "\texit\n",
                "[+] Description:\n",
                "\tUse it to exit from the program"
            ));
        }
        "listeners" | "listener" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tlisteners\n",
                "[+] Description:\n",
                "\tAccess the listeners menu"
            ));
        }
        "implants" | "implant" => {
            println!(concat!(
                "[+] Usage:\n",
                "\timplants\n",
                "[+] Description:\n",
                "\tAccess the implants menu"
            ));
        }
        _ => {}
    };
}

pub fn print_help_listeners_usage(command: &str) {
    match command {
        "back" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tback\n",
                "[+] Description:\n\t",
                "It allows you to go back to the previous menu"
            ));
        }
        "exit" => {
            println!(concat!(
                "[+] Usage:\n",
                "\texit\n",
                "[+] Description:\n",
                "\tUse it to exit from the program"
            ));
        }
        "remove" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tremove <id>\n",
                "\tremove <id1>,<id2>,...\n",
                "[+] Description:\n",
                "\tUse it to remove a stopped listener from the database"
            ));
        }
        "list" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tlist\n",
                "[+] Description:\n",
                "\tShow all the listeners in the database"
            ));
        }
        "create" => {
            println!(concat!(
                "[+] Usage:\n",
                "\tcreate\n",
                "[+] Description:\n",
                "\tAccess the menu for creating a new listener"
            ));
        }
        _ => {}
    };
}
