use chrono::format::{DelayedFormat, StrftimeItems};
use lazy_static::{lazy_static, __Deref};
use std::sync::mpsc::{Sender, channel, Receiver};
use std::time::{Duration, SystemTimeError, SystemTime};
use std::{io::Write};

mod settings;
mod models;
mod database;
mod http_server;
mod help;
mod utils;

use models::{HTTPListener, GenericListener, ListenerProtocol, ManageSettings, ListenerSignal, ImplantTask};

use crate::{http_server::start_listener, models::GenericImplant};

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

#[actix_web::main]
async fn main()
{
    database::prepare_db().unwrap();

    let mut listeners_threads_channels: Vec<(u16, Sender<ListenerSignal>)> = Vec::new();

    loop
    {
        let mut input: String = String::new();

        print!("(~)> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let split: Vec<&str> = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword: &str = split.first().unwrap().deref();

        if keyword == "exit"
        {
            break;
        }
        else if keyword == "help"
        {
            if split.get(1).is_some()
            {
                let help_argument: &str = split.get(1).unwrap().deref();

                if help_argument == "back"
                {
                    println!("[+] Usage:\n\tback");
                    println!("[+] Description:\n\tIt allows you to go back to the previous menu");
                }
                else if help_argument == "exit"
                {
                    println!("[+] Usage:\n\texit");
                    println!("[+] Description:\n\tUse it to exit from the program");
                }
                else if help_argument == "listeners" || help_argument == "listener"
                {
                    println!("[+] Usage:\n\tlisteners");
                    println!("[+] Description:\n\tAccess the listeners menu");
                }
                else if help_argument == "implants" || help_argument == "implant"
                {
                    println!("[+] Usage:\n\timplants");
                    println!("[+] Description:\n\tAccess the implants menu");
                }
            }
            else
            {
                help::print_help_main();
            }
        }
        else if keyword == "listeners" || keyword == "listener"
        {
            if process_input_listeners("listeners".to_string(), &mut listeners_threads_channels) == "exit"
            {
                break;
            };
        }
        else if keyword == "implants" || keyword == "implant"
        {
            if process_input_implants("implants".to_string()) == "exit"
            {
                break;
            }
        }
    }
}

fn process_input_listeners(
    tag: String,
    vector_thread_channels: &mut Vec<(u16, Sender<ListenerSignal>)>
) -> &'static str
{
    loop
    {
        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let split: Vec<&str> = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword: &str = split.first().unwrap().deref();

        if keyword == "back"
        {
            return "back";
        }
        else if keyword == "create"
        {
            let tmp_ret_value: &str = process_input_listeners_create("listeners/create".to_string());

            if tmp_ret_value == "exit"
            {
                return "exit";
            }
        }
        else if keyword == "exit"
        {
            return "exit"
        }
        else if keyword == "help"
        {
            if split.get(1).is_some()
            {
                let help_argument: &&str = split.get(1).unwrap();

                if *help_argument == "back"
                {
                    println!("[+] Usage:\n\tback");
                    println!("[+] Description:\n\tIt allows you to go back to the previous menu");
                }
                else if *help_argument == "exit"
                {
                    println!("[+] Usage:\n\texit");
                    println!("[+] Description:\n\tUse it to exit from the program");
                }
                else if *help_argument == "remove"
                {
                    println!("[+] Usage:\n\tremove <id>");
                    println!("\tremove <id1>,<id2>");
                    println!("\tremove <id1>,<id2>,<id3>,...");
                    println!("[+] Description:\n\tUse it to remove a stopped listener from the database");
                }
                else if *help_argument == "list"
                {
                    println!("[+] Usage:\n\tlist");
                    println!("[+] Description:\n\tShow all the listeners in the database");
                }
                else if *help_argument == "create"
                {
                    println!("[+] Usage:\n\tcreate");
                    println!("[+] Description:\n\tCreate a new listener");
                }

                continue;
            }

            help::print_help_listeners();
        }
        else if keyword == "list"
        {
            list_listeners();
        }
        else if keyword == "remove"
        {
            if split.get(1).is_none()
            {
                println!("[+] Usage:\n\tremove <id>");
                println!("\tremove <id1>,<id2>");
                continue;
            }

            let first_argument: &str = *(split.get(1).unwrap());
            let first_argument_int: Result<u16, std::num::ParseIntError> = (*first_argument).parse::<u16>();

            if first_argument_int.is_err()
            {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }
            
            let listener_id: u16 = first_argument_int.unwrap();
            if database::remove_listener(listener_id)
            {
                println!("[+] Successfully remove the listener {0}", listener_id);
            }
            else
            {
                println!("[!] Failed to remove the listener {0}", listener_id);
                println!("[?] Does it exist and is it stopped?");
            }
        }
        else if keyword == "start"
        {
            let listener_id_option: Option<&&str> = split.get(1);
            if listener_id_option.is_none()
            {
                println!("[+] Usage:\n\tstart <id>");
                continue;
            }

            let listener_id_int: Result<u16, _> = listener_id_option.unwrap().parse();
            if listener_id_int.is_err()
            {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }

            let listener_id_int_value: u16 = listener_id_int.unwrap();

            if listener_id_int_value == 0
            {
                continue;
            }

            
            let listeners: Vec<GenericListener> = crate::database::get_listeners();
            let mut listener_id_exists: bool = false;

            for generic_listener in listeners
            {
                if let ListenerProtocol::HTTP = generic_listener.protocol  
                {
                    if generic_listener.id == listener_id_int_value
                    {
                        listener_id_exists = true;
                        break;
                    }
                }
            }

            if listener_id_exists
            {
                let (tx, rx): (Sender<ListenerSignal>, Receiver<ListenerSignal>) = channel();            

                std::thread::spawn(move || start_listener(listener_id_int_value, rx));
    
                vector_thread_channels.push((listener_id_int_value, tx));
            }
            else {
                println!("[!] The identifier you specified is not associated with any listeners");
            }
        }
        else if keyword == "stop"
        {
            let listener_id_option: Option<&&str> = split.get(1);
            if listener_id_option.is_none()
            {
                println!("[+] Usage:\n\tstop <id>");
                continue;
            }

            let listener_id_int: Result<u16, _> = listener_id_option.unwrap().parse();
            if listener_id_int.is_err()
            {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }

            let listener_id_int_value: u16 = listener_id_int.unwrap();
            
            if crate::database::set_listener_state(listener_id_int_value, models::ListenerState::Suspended)
            {
                println!("[+] Changed the state of the listener");
            }
            else
            {
                println!("[!] Failed to change the state of the listener");
            }

            let thread_join_handle_index: Option<usize> = vector_thread_channels.iter().position(
                |x| x.0 == listener_id_int_value
            );

            if thread_join_handle_index.is_some()
            {
                let vector_element: (u16, Sender<ListenerSignal>) = vector_thread_channels.remove(thread_join_handle_index.unwrap());
                let tx = vector_element.1;

                match tx.send(ListenerSignal::StopListener)
                {
                    Ok(_) => {
                        println!("[+] Sent STOP signal to the listener")
                    },
                    Err(_) => {
                        println!("[!] Couldn't send the stop signal to the listener");
                    }
                }
            }
            else
            {
                println!("[!] Failed to retrieve to join handle of the listener's thread")
            }
        }
    }
}

fn process_input_implants(tag: String) -> &'static str
{
    loop
    {
        let mut input: String = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let split: Vec<&str> = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword: &str = &split.first().unwrap();

        if keyword == "back"
        {
            return "back";
        }
        else if keyword == "exit"
        {
            return "exit"
        }
        else if keyword == "help"
        {
            help::print_help_implants();
        }
        else if keyword == "interact"
        {
            if split.get(1).is_none()
            {
                println!("[+] Usage:\n\tinteract <id>");
                continue;
            }

            let first_argument: &str = *(split.get(1).unwrap());
            let first_argument_int: Result<u16, std::num::ParseIntError> = (*first_argument).parse::<u16>();

            if first_argument_int.is_err()
            {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }
            
            let implant_id: u16 = first_argument_int.unwrap();

            if ! database::check_if_implant_exists(Some(implant_id), None)
            {
                println!("[!] There's no implant indentified by this ID");
                continue;
            }

            let new_tag = format!("implants/{}", implant_id);
            let tmp_ret_value: &str = process_input_implants_interact(implant_id, new_tag.to_string());

            if tmp_ret_value == "exit"
            {
                return "exit";
            }
        }
        else if keyword == "list"
        {
            list_implants();
        }
        else if keyword == "remove"
        {
            if split.get(1).is_none()
            {
                println!("[+] Usage:\n\tremove <id>");
                println!("\tremove <id1>,<id2>");
                continue;
            }

            let first_argument: &str = *(split.get(1).unwrap());
            let first_argument_int: Result<u16, std::num::ParseIntError> = (*first_argument).parse::<u16>();

            if first_argument_int.is_err()
            {
                println!("[!] Couldn't convert the parameter to an integer");
                continue;
            }
            
            let implant_id: u16 = first_argument_int.unwrap();
            if database::remove_implant(implant_id)
            {
                println!("[+] Successfully removed the implant {0}", implant_id);
            }
            else
            {
                println!("[!] Failed to remove the implant {0}", implant_id);
            }
        }
        else if keyword == "tasks"
        {
            let implant_tasks = database::get_all_tasks(true);
            list_tasks(implant_tasks);
        }
    }
}

fn process_input_listeners_create(tag: String) -> &'static str
{
    let address: &String = &CONFIG.listener.address;
    let port: u16 = CONFIG.listener.port;

    let mut http_listener: HTTPListener;
    
    match HTTPListener::create(address.clone(), port)
    {
        Ok(v) => {
            http_listener = v;
        },
        Err(_) => {
            println!("[!] Invalid address/port for the listener. Check the default profile");
            return "exit";
        }
    }

    loop
    {
        let mut input: String = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let split: Vec<&str> = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword: &&str = split.first().unwrap();
        // println!("[#] Keyword: '{0}'", keyword);

        match *keyword
        {
            "back" =>
            {
                return "back";
            }
            "create" =>
            {
                HTTPListener::add_to_database(http_listener);
                println!("[+] Listener created successfully");
                return "back";
            }
            "exit" =>
            {
                return "exit";
            }
            "help" =>
            {
                help::print_help_listeners_create();
            }
            "options" =>
            {
                http_listener.show_settings();
            }
            "set" =>
            {
                let param1: Option<&&str> = split.get(1);
                let param2: Option<&&str> = split.get(2);

                if param1.is_none() || param2.is_none()
                {
                    println!("[+] Usage:\n\tset <option> <value>");
                    continue;
                }

                let option: &str = *(param1.unwrap());
                let value: &str = *(param2.unwrap());

                match http_listener.set_option(option, value)
                {
                    true =>
                    {
                        println!("[+] Successfully set the listener option\n");
                        http_listener.show_settings();
                    }
                    false =>
                    {
                        println!("[!] Failed to set the listener option");
                    }
                }
            }
            _ => {}
        }
    }
}

fn list_listeners() -> Vec<GenericListener>
{
    let listeners: Vec<GenericListener> = crate::database::get_listeners();  

    if listeners.is_empty()
    {
        println!("[+] No listeners found");
        return listeners;
    }

    println!("+----+------------+-----------------+-------+");
    println!("| ID |   STATE    |     ADDRESS     |  PORT |");
    println!("+----+------------+-----------------+-------+");

    for listener in listeners.iter()
    {

        if let ListenerProtocol::HTTP = listener.protocol
        {
            let http_listener: &HTTPListener = listener.data.downcast_ref::<HTTPListener>().unwrap();

            println!(
                "| {0:^2} | {1:^10} | {2:^15} | {3:^5} |",
                listener.id,
                listener.state,
                http_listener.address,
                http_listener.port
            );
        }
    }

    println!("+----+------------+-----------------+-------+");

    return listeners;
}

fn list_implants()
{
    let implants: Vec<GenericImplant> = database::get_implants();  

    if implants.is_empty()
    {
        println!("[+] No implants found");
        return;
    }

    println!("+----+------------+-----------------+");
    println!("| ID |  Listener  |    Last Seen    |");
    println!("+----+------------+-----------------+");

    let time_elapsed_now: Result<Duration, SystemTimeError> = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let time_now_seconds: u64 = time_elapsed_now.as_ref().unwrap().as_secs();

    for implant in implants
    {
        let last_seen_string: String = if ! time_elapsed_now.is_err()
        {
            let time_diff_seconds = time_now_seconds - implant.last_seen;

            if time_diff_seconds <= 59 {
                format!("{}s ago", time_diff_seconds)
            }
            else if time_diff_seconds <= 3599
            {
                format!("{}m {}s ago", time_diff_seconds / 60, time_diff_seconds % 60)
            }
            else
            {
                format!("{}h {}m ago", time_diff_seconds / 3600, (time_diff_seconds % 3600 ) / 60)
            }
        }
        else
        {
            format!("{}", implant.last_seen)
        };

        println!(
            "| {0:^2} | {1:^10} | {2:^15} |",
            implant.id,
            implant.listener_id,
            last_seen_string,
        );
    }

    println!("+----+------------+-----------------+");
}

fn process_input_implants_interact(implant_id: u16, tag: String) -> &'static str
{
    loop
    {
        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let split: Vec<&str> = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword: &str = split.first().unwrap().deref();

        if keyword == "back"
        {
            return "back";
        }
        else if keyword == "exit"
        {
            return "exit"
        }
        else if keyword == "help"
        {
            help::print_help_implants_interaction();
        }
        else if keyword == "tasks"
        {
            let implant_tasks = database::get_implant_tasks(implant_id, true);
            list_tasks(implant_tasks);
        }
        else if keyword == "whoami"
        {

            let mut new_task_command = keyword;
            
            for implant_task_command in &CONFIG.implant.tasks.commands
            {
                if implant_task_command.name != keyword
                {
                    continue
                }
                
                if CONFIG.implant.tasks.use_commands_codes
                {
                    new_task_command = &implant_task_command.code;
                } 
                else if CONFIG.implant.tasks.use_alt_names
                {
                    new_task_command = &implant_task_command.alt_name;
                }

                if database::create_implant_task(implant_id, new_task_command)
                {
                    println!("[+] Task issued successfully");
                    break;
                }
            }
        }
        else
        {
            continue;
        }
    }
}

fn list_tasks(tasks: Vec<ImplantTask>) {

    if tasks.len() == 0
    {
        println!("[+] No tasks found");
        return;
    }

    println!("+------+------------------------+-----------------+");
    println!("|  ID  |        Date time       |      Status     |");
    println!("+------+------------------------+-----------------+");

    for task in tasks
    {
        let formatted_date_time: DelayedFormat<StrftimeItems> = utils::format_date_time(task.datetime, "%Y-%m-%d %H:%M:%S");

        println!(
            "| {0:^4} | {1:^20}+0 | {2:^15} |",
            task.id,
            formatted_date_time,
            task.status
        );
    }

    println!("+------+------------------------+-----------------+");
}