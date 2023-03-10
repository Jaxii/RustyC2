use chrono::format::{DelayedFormat, StrftimeItems};
use lazy_static::{__Deref, lazy_static};
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};

mod database;
mod implants;
mod misc;
mod models;
mod servers;
mod settings;

use models::{HTTPListener, ImplantTask, ListenerProtocol, ListenerSignal, ManageSettings};

use crate::misc::help;
use crate::models::{EnumImplantCommands, EnumImplantTaskCommands, ImplantTaskStatus};
use crate::servers::http::start_listener;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new();
}

#[actix_web::main]
async fn main() {
    if database::prepare_db().is_err() {
        println!("[!] Failed to set up the database");
    }

    let mut listeners_threads_channels: Vec<(u16, Sender<ListenerSignal>)> = Vec::new();

    loop {
        let mut input: String = String::new();

        print!("({})> ", &CONFIG.client.main_tag);

        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let split: Vec<&str> = input
            .as_str()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if split.first().is_none() {
            continue;
        }

        let keyword: &str = &split.first().unwrap().deref().to_lowercase();

        if keyword == "exit" {
            break;
        } else if keyword == "help" {
            match split.get(1) {
                Some(help_argument) => {
                    misc::help::print_help_usage(&*help_argument.to_lowercase());
                }
                None => {
                    misc::help::print_help_main();
                }
            }
        } else if keyword == "listeners" || keyword == "listener" {
            if process_input_listeners("listeners".to_string(), &mut listeners_threads_channels)
                == "exit"
            {
                break;
            };
        } else if keyword == "implants" || keyword == "implant" {
            if process_input_implants("implants".to_string()) == "exit" {
                break;
            }
        }
    }
}

fn process_input_listeners(
    tag: String,
    vector_thread_channels: &mut Vec<(u16, Sender<ListenerSignal>)>,
) -> &'static str {
    loop {
        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let split: Vec<&str> = input
            .as_str()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if split.first().is_none() {
            continue;
        }

        let keyword: &str = &split.first().unwrap().deref().to_lowercase();

        if keyword == "back" {
            return "back";
        } else if keyword == "create" {
            let tmp_ret_value: &str =
                process_input_listeners_create("listeners/create".to_string());

            if tmp_ret_value == "exit" {
                return "exit";
            }
        } else if keyword == "exit" {
            return "exit";
        } else if keyword == "help" {
            match split.get(1) {
                Some(help_argument) => {
                    misc::help::print_help_listeners_usage(&*help_argument.to_lowercase());
                }
                None => {
                    misc::help::print_help_listeners();
                }
            };

            continue;
        } else if keyword == "list" {
            misc::utils::list_listeners();
        } else if keyword == "remove" {
            if split.get(1).is_none() {
                println!("[+] Usage:\n\tremove <id>");
                println!("\tremove <id1>,<id2>");
                continue;
            }

            let first_argument: &str = *(split.get(1).unwrap());
            let first_argument_int: Result<u16, std::num::ParseIntError> =
                (*first_argument).parse::<u16>();

            if first_argument_int.is_err() {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }

            let listener_id: u16 = first_argument_int.unwrap();
            if database::remove_listener(listener_id) {
                println!("[+] Successfully removed the listener {0}", listener_id);
            } else {
                println!("[!] Failed to remove the listener {0}", listener_id);
                println!("[?] Does it exist and is it stopped?");
            }
        } else if keyword == "start" {
            let listener_id_option: Option<&&str> = split.get(1);
            if listener_id_option.is_none() {
                println!("[+] Usage:\n\tstart <id>");
                continue;
            }

            let listener_id_int: Result<u16, _> = listener_id_option.unwrap().parse();
            if listener_id_int.is_err() {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }

            let listener_id_int_value: u16 = listener_id_int.unwrap();

            if listener_id_int_value == 0 {
                continue;
            }

            let listeners: Vec<HTTPListener> = database::get_http_listeners();
            let mut listener_id_exists: bool = false;

            for generic_listener in listeners {
                if let ListenerProtocol::HTTP = generic_listener.protocol {
                    if generic_listener.id == listener_id_int_value {
                        listener_id_exists = true;
                        break;
                    }
                }
            }

            if listener_id_exists {
                let (tx, rx): (Sender<ListenerSignal>, Receiver<ListenerSignal>) = channel();

                std::thread::spawn(move || start_listener(listener_id_int_value, rx));

                vector_thread_channels.push((listener_id_int_value, tx));
            } else {
                println!("[!] The identifier you specified is not associated with any listeners");
            }
        } else if keyword == "stop" {
            let listener_id_option: Option<&&str> = split.get(1);
            if listener_id_option.is_none() {
                println!("[+] Usage:\n\tstop <id>");
                continue;
            }

            let listener_id_int: Result<u16, _> = listener_id_option.unwrap().parse();
            if listener_id_int.is_err() {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }

            let listener_id_int_value: u16 = listener_id_int.unwrap();

            if crate::database::set_listener_status(
                listener_id_int_value,
                models::ListenerStatus::Suspended,
            ) {
                println!("[+] Changed the status of the listener");
            } else {
                println!("[!] Failed to change the status of the listener");
            }

            let thread_join_handle_index: Option<usize> = vector_thread_channels
                .iter()
                .position(|x| x.0 == listener_id_int_value);

            if thread_join_handle_index.is_some() {
                let vector_element: (u16, Sender<ListenerSignal>) =
                    vector_thread_channels.remove(thread_join_handle_index.unwrap());
                let tx = vector_element.1;

                match tx.send(ListenerSignal::StopListener) {
                    Ok(_) => {
                        println!("[+] Sent STOP signal to the listener")
                    }
                    Err(_) => {
                        println!("[!] Couldn't send the stop signal to the listener");
                    }
                }
            } else {
                println!("[!] Failed to retrieve to join handle of the listener's thread")
            }
        }
    }
}

fn process_input_implants(tag: String) -> &'static str {
    loop {
        let mut input: String = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let split: Vec<&str> = input
            .as_str()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if split.first().is_none() {
            continue;
        }

        let keyword: &str = &split.first().unwrap().to_lowercase();

        if EnumImplantCommands::Back == keyword {
            return "back";
        } else if EnumImplantCommands::Exit == keyword {
            return "exit";
        } else if EnumImplantCommands::Generate == keyword {
            let listener_id_option: Option<&&str> = split.get(1);
            if listener_id_option.is_none() || split.len() < 3 {
                help::print_help_implants(Some(EnumImplantCommands::Generate));
                continue;
            }

            match listener_id_option.unwrap().parse::<u16>() {
                Ok(listener_id) => match database::get_listener_protocol(listener_id) {
                    Some(listener_protocol) => match listener_protocol {
                        ListenerProtocol::TCP => todo!(),
                        ListenerProtocol::UDP => todo!(),
                        ListenerProtocol::HTTP => match database::get_http_listener(listener_id) {
                            Some(http_listener) => match split.get(2) {
                                Some(implant_project_name) => {
                                    implants::generate::generate_http_implant(
                                        http_listener,
                                        &implant_project_name.to_ascii_lowercase(),
                                    );
                                }
                                None => {
                                    println!("[!] Failed to get the name of the implant project")
                                }
                            },
                            None => {
                                println!("[!] Couldn't retrieve the HTTP data of the listener")
                            }
                        },
                        ListenerProtocol::ICMP => todo!(),
                        ListenerProtocol::DNS => todo!(),
                    },
                    None => {
                        println!("[!] Couldn't retrieve the protocol of the listener specified")
                    }
                },
                Err(_) => {
                    println!("[!] Couldn't convert the parameter to an integer");
                    continue;
                }
            }
        } else if EnumImplantCommands::Help == keyword {
            match split.get(1) {
                None => help::print_help_implants(None),
                Some(help_argument) => match help_argument.parse::<EnumImplantCommands>() {
                    Ok(implant_command) => {
                        help::print_help_implants(Some(implant_command));
                    }
                    Err(_) => {
                        help::print_help_implants(Some(EnumImplantCommands::Help));
                    }
                },
            }
        } else if EnumImplantCommands::Interact == keyword {
            match split.get(1) {
                Some(interact_argument) => {
                    let first_argument_int: Result<u16, std::num::ParseIntError> =
                        (*interact_argument).parse::<u16>();

                    if first_argument_int.is_err() {
                        println!("[!] Couldn't convert the parameter to an integer");
                        continue;
                    }

                    let implant_id: u16 = first_argument_int.unwrap();

                    if database::check_if_implant_exists(Some(implant_id), None).is_none() {
                        println!("[!] There's no implant indentified by this ID");
                        continue;
                    }

                    let new_tag = format!("implants/{}", implant_id);
                    let tmp_ret_value: &str =
                        process_input_implants_interact(implant_id, new_tag.to_string());

                    if tmp_ret_value == "exit" {
                        return tmp_ret_value;
                    }
                }
                None => help::print_help_implants(Some(EnumImplantCommands::Interact)),
            }
        } else if EnumImplantCommands::List == keyword {
            misc::utils::list_implants();
        } else if EnumImplantCommands::Remove == keyword {
            match split.get(1) {
                Some(first_argument) => {
                    let first_argument_int: Result<u16, std::num::ParseIntError> =
                        (*first_argument).parse::<u16>();

                    if first_argument_int.is_err() {
                        println!("[!] Couldn't convert the parameter to an integer");
                        continue;
                    }

                    let implant_id: u16 = first_argument_int.unwrap();
                    if database::remove_implant(implant_id) {
                        println!("[+] Successfully removed the implant {0}", implant_id);
                    } else {
                        println!("[!] Failed to remove the implant {0}", implant_id);
                    }
                }
                None => {
                    help::print_help_implants(Some(EnumImplantCommands::Remove));
                }
            }
        } else if EnumImplantCommands::Tasks == keyword {
            let implant_tasks = database::get_all_tasks(true);
            list_tasks(implant_tasks);
        }
    }
}

fn process_input_listeners_create(tag: String) -> &'static str {
    let address: &String = &CONFIG.listener.http.address;
    let port: u16 = CONFIG.listener.http.port;

    let mut http_listener: HTTPListener;

    match HTTPListener::create(address.clone(), port) {
        Ok(v) => {
            http_listener = v;
        }
        Err(_) => {
            println!("[!] Invalid address/port for the listener. Check the default profile");
            return "exit";
        }
    }

    loop {
        let mut input: String = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let split: Vec<&str> = input
            .as_str()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if split.first().is_none() {
            continue;
        }

        let keyword: &&str = split.first().unwrap();

        match *keyword {
            "back" => {
                return "back";
            }
            "create" => {
                HTTPListener::add_to_database(http_listener);
                println!("[+] Listener created successfully");
                return "back";
            }
            "exit" => {
                return "exit";
            }
            "help" => {
                misc::help::print_help_listeners_create();
            }
            "options" => {
                http_listener.show_settings();
            }
            "set" => {
                let param1: Option<&&str> = split.get(1);
                let param2: Option<&&str> = split.get(2);

                if param1.is_none() || param2.is_none() {
                    println!("[+] Usage:\n\tset <option> <value>");
                    continue;
                }

                let option: &str = *(param1.unwrap());
                let value: &str = *(param2.unwrap());

                match http_listener.set_option(option, value) {
                    true => {
                        println!("[+] Successfully set the listener option\n");
                        http_listener.show_settings();
                    }
                    false => {
                        println!("[!] Failed to set the listener option");
                    }
                }
            }
            _ => {}
        }
    }
}

fn process_input_implants_interact(implant_id: u16, tag: String) -> &'static str {
    loop {
        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let split: Vec<&str> = input
            .as_str()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if split.first().is_none() {
            continue;
        }

        let keyword: &str = &split.first().unwrap().deref().to_lowercase();

        if keyword == "back" {
            return "back";
        } else if keyword == "exit" {
            return "exit";
        } else if keyword == "help" {
            misc::help::print_help_implants_interaction(None);
        } else if keyword == "tasks" {
            if split.get(1).is_none() {
                list_tasks(database::get_implant_tasks(
                    "Id",
                    implant_id.to_string().as_str(),
                    vec![
                        ImplantTaskStatus::Issued.to_string(),
                        ImplantTaskStatus::Pending.to_string(),
                        ImplantTaskStatus::Completed.to_string(),
                    ],
                ));

                continue;
            }

            match *(split.get(1).unwrap()) {
                "get" => match split.get(2) {
                    Some(task_id) => match task_id.parse::<u64>() {
                        Ok(task_id_int) => match database::get_task(task_id_int) {
                            Some(implant_task) => {
                                if implant_task.output.is_empty() {
                                    println!("[+] There's no output yet");
                                } else {
                                    match String::from_utf8(implant_task.output) {
                                        Ok(task_output) => {
                                            println!(
                                                "[+] Output of the task {}:\n{}",
                                                implant_task.id, task_output
                                            );
                                        }
                                        Err(_) => {
                                            println!("[!] Couldn't print the output of the command to screen");
                                        }
                                    };
                                }
                            }
                            None => {
                                println!("[!] Failed to retrieve the task");
                            }
                        },
                        Err(_) => {
                            println!("[!] Failed to parse the ID of the task to retrieve");
                        }
                    },
                    None => {
                        println!(concat!(
                            "[+] Usage:\n",
                            "\ttasks get <id>\n",
                            "[+] Description:\n",
                            "\tRetrieve the output of a specific task"
                        ));
                    }
                },
                "list" => {
                    list_tasks(database::get_implant_tasks(
                        "Id",
                        implant_id.to_string().as_str(),
                        vec![
                            ImplantTaskStatus::Issued.to_string(),
                            ImplantTaskStatus::Pending.to_string(),
                            ImplantTaskStatus::Completed.to_string(),
                        ],
                    ));
                }
                "remove" => {}
                _ => {}
            };

            continue;
        } else if EnumImplantTaskCommands::Whoami == keyword
            || EnumImplantTaskCommands::Pwd == keyword
        {
            let mut new_task_command: &str = keyword;

            for implant_task_command in &CONFIG.implant.tasks.commands {
                if implant_task_command.name != keyword || !implant_task_command.enabled {
                    continue;
                }

                if CONFIG.implant.tasks.use_commands_codes {
                    new_task_command = &implant_task_command.code;
                } else if CONFIG.implant.tasks.use_alt_names {
                    new_task_command = &implant_task_command.alt_name;
                }

                if database::create_implant_task(implant_id, new_task_command, None) {
                    println!("[+] Task issued successfully");
                    break;
                }
            }
        } else if EnumImplantTaskCommands::InjectLocal == keyword {
            if split.get(1).is_none() {
                help::print_help_implants_interaction(None);
                continue;
            }

            let mut new_task_command: &str = keyword;

            for implant_task_command in &CONFIG.implant.tasks.commands {
                if implant_task_command.name != keyword || !implant_task_command.enabled {
                    continue;
                }

                if CONFIG.implant.tasks.use_commands_codes {
                    new_task_command = &implant_task_command.code;
                } else if CONFIG.implant.tasks.use_alt_names {
                    new_task_command = &implant_task_command.alt_name;
                }

                if database::create_implant_task(implant_id, new_task_command, None) {
                    println!("[+] Task issued successfully");
                    break;
                }
            }
        } else if EnumImplantTaskCommands::InjectRemote == keyword {
        } else {
            continue;
        }
    }
}

fn list_tasks(tasks: Vec<ImplantTask>) {
    if tasks.len() == 0 {
        println!("[+] No tasks found");
        return;
    }

    println!("+------+------------------------+-----------------+");
    println!("|  ID  |        Date time       |      Status     |");
    println!("+------+------------------------+-----------------+");

    for task in tasks {
        let formatted_date_time: DelayedFormat<StrftimeItems> =
            misc::utils::format_date_time(task.datetime, "%Y-%m-%d %H:%M:%S");

        println!(
            "| {0:^4} | {1:^20}+0 | {2:^15} |",
            task.id, formatted_date_time, task.status.to_string()
        );
    }

    println!("+------+------------------------+-----------------+");
}
