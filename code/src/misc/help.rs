pub fn print_help_main()
{
    let help_items = [
        ("exit",        "Exit from the framework"),
        ("help",        "Show this help menu"),
        ("implants",    "Manage implants"),
        ("listeners",   "Manage listeners"),
    ];

    println!();
    println!("{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }

    println!();
}

pub fn print_help_implants()
{
    let help_items = [
        ("back",        "Return to the main menu"),
        ("exit",        "Exit from the framework"),
        ("generate",    "Generate implant code"),
        ("help",        "Show this help menu"),
        ("list",        "List all the implants"),
        ("interact",    "Interact with a specific implant"),
        ("kill",        "Kill implant"),
        ("remove",      "Remove an implant"),
        ("sleep",       "Change callback delay"),
    ];

    println!("\n{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }
    println!();
}

pub fn print_help_listeners()
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

pub fn print_help_listeners_create()
{
    let help_items = [
        ("back",    "Return to the previous menu"),
        ("create",  "Create a new listener"),
        ("exit",    "Exit from the framework"),
        ("help",    "Show this help menu"),
        ("options", "Show options"),
        ("set",     "Change listener settings"),
    ];

    println!();
    println!("{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }

    println!();
}

pub fn print_help_implants_interaction()
{
    let help_items = [
        ("back",        "Return to the previous menu"),
        ("exit",        "Exit from the framework"),
        ("hostname",    "Show the name of the host"),
        ("info",        "Show info about the implant"),
        ("ls",          "Show files/directories in the current path"),
        ("addresses",   "Show the IP address of the host"),
        ("pwd",         "Show the current path"),
        ("tasks",       "Manage tasks"),
        ("whoami",      "Print the current username"),
    ];

    println!();
    println!("{0: <20}{1}", "Command", "Description");
    println!("{0: <20}{1}", "-------", "-----------");

    for item in help_items {
        println!("{0: <20}{1}", item.0, item.1);
    }

    println!();
}

pub fn print_help_usage(command: &str)
{
    match command
    {
        "back" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\tback\n",
                "[+] Description:\n\t",
                "It allows you to go back to the previous menu"
            ));
        },
        "exit" => 
        {
            println!(concat!(
                "[+] Usage:\n",
                "\texit\n",
                "[+] Description:\n",
                "\tUse it to exit from the program"
            ));
        },
        "listeners"|"listener" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\tlisteners\n",
                "[+] Description:\n",
                "\tAccess the listeners menu"
            ));
        }
        "implants" | "implant" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\timplants\n",
                "[+] Description:\n",
                "\tAccess the implants menu"
            ));
        },
        _ => {}
    };
}

pub fn print_help_listeners_usage(command: &str)
{
    match command
    {
        "back" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\tback\n",
                "[+] Description:\n\t",
                "It allows you to go back to the previous menu"
            ));
        },
        "exit" => 
        {
            println!(concat!(
                "[+] Usage:\n",
                "\texit\n",
                "[+] Description:\n",
                "\tUse it to exit from the program"
            ));
        },
        "remove" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\tremove <id>\n",
                "\tremove <id1>,<id2>,...\n",
                "[+] Description:\n",
                "\tUse it to remove a stopped listener from the database"
            ));
        }
        "list" =>
        {
            println!(concat!(
                "[+] Usage:\n",
                "\tlist\n",
                "[+] Description:\n",
                "\tShow all the listeners in the database"
            ));
        },
        "create" =>
        {
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
