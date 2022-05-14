use lazy_static::lazy_static;
use std::io::{Write};

mod settings;
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
        let mut input = String::new();

        print!("(~)> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.as_str().trim()
        {
            "exit" => break,
            "help" => print_help_main(),
            "listeners" => match process_input_listeners("listeners".to_string())
            {
                "exit" => break,
                _ => ()
            },
            "implants" => match process_input_implants("implants".to_string())
            {
                "exit" => break,
                _ => ()
            }
            _ => ()
        }
    }
}

fn process_input_listeners(tag: String) -> &'static str
{
    loop
    {
        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let input_trimmed = input.as_str().trim();

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
            print_help_listeners();
        }
        else if input_trimmed == "create"
        {
            let tmp_ret_value = process_input_listeners_create("listeners/create".to_string());

            if tmp_ret_value == "exit"
            {
                return "exit";
            }
        }
    }
}

fn process_input_implants(tag: String) -> &'static str
{
    loop
    {
        let mut input = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input_trimmed = input.as_str().trim();

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
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("implants", "Manage implants"),
        ("listeners", "Manage listeners"),
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
        ("back", "Return to the main menu"),
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("interact", "Interact with a specific implant"),
        ("kill", "Kill implant"),
        ("sleep", "Change callback delay"),
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
        ("back", "Return to the main menu"),
        ("create", "Create a new listener"),
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("kill", "Kill a specific listener"),
        ("update", "Change settings of a listeners"),
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
    let address = &CONFIG.listener.address;
    let port = CONFIG.listener.port;

    loop
    {
        let mut input = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let split = input.as_str().trim().split_whitespace().collect::<Vec<&str>>();

        if split.first().is_none()
        {
            continue;
        }

        let keyword = split.first().unwrap();
        // println!("[#] Keyword: '{0}'", keyword);

        if *keyword == "back"
        {
            return "back";
        }
        else if *keyword == "create" {
            std::thread::spawn(move || {
                http_server::create(address.to_string(), port)
            });
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
        ("back", "Return to the previous menu"),
        ("create", "Create a new listener"),
        ("exit", "Exit from the framework"),
        ("help", "Show this help menu"),
        ("set", "Change listener settings"),
        ("start", "Create a new listener and start it"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}
