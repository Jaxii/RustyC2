use lazy_static::lazy_static;
use std::io::Write;

use crate::models::{HTTPListener, Listener};

mod settings;
mod models;
mod database;
mod http_server;

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

#[actix_web::main]
async fn main()
{
    database::prepare_db().unwrap();

    loop
    {
        let mut input: String = String::new();

        print!("(~)> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let input_trimmed: &str = input.as_str().trim();

        if input_trimmed == "exit"
        {
            break;
        }
        else if input_trimmed == "help"
        {
            print_help_main();
        }
        else if input_trimmed == "listeners"
        {
            if process_input_listeners("listeners".to_string()) == "exit"
            {
                break;
            };
        }
        else if input_trimmed == "implants"
        {
            if process_input_implants("implants".to_string()) == "exit"
            {
                break;
            }
        }
    }
}

fn process_input_listeners(tag: String) -> &'static str
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

        let keyword: &&str = split.first().unwrap();

        if *keyword == "back"
        {
            return "back";
        }
        else if *keyword == "create"
        {
            let tmp_ret_value: &str = process_input_listeners_create("listeners/create".to_string());

            if tmp_ret_value == "exit"
            {
                return "exit";
            }
        }
        else if *keyword == "exit"
        {
            return "exit"
        }
        else if *keyword == "help"
        {
            print_help_listeners();
        }
        else if *keyword == "list"
        {
            list_listeners();
        }
        else if *keyword == "remove"
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
        let input_trimmed: &str = input.as_str().trim();

        if input_trimmed == "back"
        {
            return "back";
        }
        else if input_trimmed == "exit"
        {
            return "exit"
        }
        else if input_trimmed == "help"
        {
            print_help_implants();
        }
        
    }
}

fn print_help_main()
{
    let help_items = [
        ("exit",        "Exit from the framework"),
        ("help",        "Show this help menu"),
        ("implants",    "Manage implants"),
        ("listeners",   "Manage listeners"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

fn print_help_implants()
{
    let help_items = [
        ("back",        "Return to the main menu"),
        ("exit",        "Exit from the framework"),
        ("help",        "Show this help menu"),
        ("list",        "List all the implants"),
        ("interact",    "Interact with a specific implant"),
        ("kill",        "Kill implant"),
        ("sleep",       "Change callback delay"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

fn print_help_listeners()
{
    let help_items = [
        ("back",    "Return to the main menu"),
        ("create",  "Create a new listener"),
        ("exit",    "Exit from the framework"),
        ("help",    "Show this help menu"),
        ("list",    "List all the listeners"),
        ("remove",  "Remove a listener"),
        ("start",   "Start/resume a specific listener"),
        ("stop",    "Suspend a specific listener"),
        ("update",  "Change settings of a listeners"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

fn process_input_listeners_create(tag: String) -> &'static str
{
    let address: &String = &CONFIG.listener.address;
    let port: u16 = CONFIG.listener.port;

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

        if *keyword == "back"
        {
            return "back";
        }
        else if *keyword == "create"
        {
            let flag: bool = HTTPListener::create(address.to_string(), port);
            
            if flag
            {
                println!("[+] Listener created successfully");
                return "back";
            }
            else
            {
                println!("[!] An error occured while creating the listener");
            }
        }
        else if *keyword == "exit"
        {
            return "exit";
        }
        else if *keyword == "help"
        {
            print_help_listeners_create();
        }
    }
}

fn print_help_listeners_create()
{
    let help_items = [
        ("back",    "Return to the previous menu"),
        ("create",  "Create a new listener"),
        ("exit",    "Exit from the framework"),
        ("help",    "Show this help menu"),
        ("set",     "Change listener settings"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

fn list_listeners()
{
    let listeners: Vec<Listener> = crate::database::get_listeners();  

    if listeners.is_empty()
    {
        println!("[+] No listeners found");
        return;
    }

    println!("+----+------------+-----------------+-------+------------+");
    println!("| ID |   STATE    |     ADDRESS     |  PORT |  PROTOCOL  |");
    println!("+----+------------+-----------------+-------+------------+");

    for listener in listeners
    {
        println!(
            "| {0:^2} | {1:^10} | {2:^15} | {3:^5} | {4:^10} |",
            listener.id,
            listener.state,
            listener.address,
            listener.port,
            listener.protocol
        );
    }
    println!("+----+------------+-----------------+-------+------------+");
}
