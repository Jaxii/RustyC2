// use actix_web::{web, App, HttpResponse, HttpServer};
use lazy_static::lazy_static;
use std::io::{Write};

mod settings;

lazy_static!
{
    static ref CONFIG: settings::Settings =
        settings::Settings::new().unwrap();
}

fn main()
{
    // // Create HTTP listener for new agents
    // let test_listener = HttpServer::new(|| App::new().service(web::resource("/").to(|| HttpResponse::Ok())));

    // // Start HTTP listener
    // let listener_addr = String::from(&format!(
    //     "{}:{}",
    //     CONFIG.listener.address, CONFIG.listener.port
    // ));
    // test_listener.bind(listener_addr)?
    //     .run()
    //     .await;

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
            "listeners" => process_input_listeners("listeners".to_string()),
            "implants" => process_input_implants("implants".to_string()),
            _ => ()
        }
    }
}

fn process_input_listeners(tag: String)
{
    loop
    {
        let mut input = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        // println!("> {}", b1);

        match input.as_str().trim()
        {
            "back" => break,
            _ => ()
        }
    }
}

fn process_input_implants(tag: String)
{
    loop
    {
        let mut input = String::new();

        print!("({})> ", tag);
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        // println!("> {}", b1);

        match input.as_str().trim()
        {
            "back" => break,
            _ => ()
        }
    }
}

fn print_help_main()
{

}