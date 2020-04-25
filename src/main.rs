#![warn(clippy::all)]
// TODO: Authentication Portal

use async_std::task;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

mod auth;
mod api;
mod in_memory_db;

fn usage() {
    let display_options = r#"
    -h     <this message>
    -v     Show version"#;

    println!("Usage: {} [OPTIONS]...", PROGRAM_NAME);
    println!("Program description...");
    println!("{}", display_options);
}

// Example: HTTP GET to http://localhost:8080/fib/42
// $ curl "http://localhost:8080/fib/42"
// The fib of 42 is 267914296.
// It was computed in 2 secs.
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        use in_memory_db::State;
        use tide::Server;
        let listen_addr = "0.0.0.0";
        let listen_port = "8081";
        let bind = format!("{}:{}", listen_addr, listen_port);
        task::block_on(async {
            let mut app = Server::with_state(State::new());
            app.at("/").get(api::presenter::index_page);
            // app.at("/login").get(auth::show_login);
            // app.at("/login").post(auth::submit_login);
            // app.at("/logout").get(auth::logout);
            // app.at("/uptime").get(api::presenter::show_uptime);
            // app.at("/processes").get(api::presenter::show_processes);
            // app.at("/interfaces").get(api::presenter::show_interfaces);
            // app.at("/netstat").get(api::presenter::show_netstat);
            // app.at("/disk").get(api::presenter::show_disk);
            // app.at("/printenv").get(api::presenter::printenv);
            // app.at("/memory").get(api::presenter::show_memory);
            // app.at("/disk").get(api::presenter::show_disk);
            // app.at("/cpuinfo").get(api::presenter::cpuinfo);
            // app.at("/show-lsof").get(api::presenter::show_lsof);
            // app.at("/show-root").get(api::presenter::show_root);
            // app.at("/show-pwd").get(api::presenter::show_pwd);
            // app.at("/cpuinfo").get(api::presenter::cpuinfo);
            // app.at("/404").get(auth::show_404);
            // app.at("/404").post(auth::show_404);
            // // app.at("/fib/:n").get(fibsum);
            // app.at("/fib").get(api::presenter::fibsum);

            print!("{} v{} - ", PROGRAM_NAME, VERSION);
            println!("Listening at: http://{}", &bind);
            app.listen(bind).await?;
            std::process::exit(0);
        })
    } else if args.len() == 2 && !args[1].is_empty() {
        match args[1].as_ref() {
            "-v" => {
                println!("{} v{}", PROGRAM_NAME, VERSION);
                std::process::exit(0);
            }
            "-h" => {
                usage();
                std::process::exit(1);
            }
            _ => {
                println!("Not implemented: [{}]\n", args[1]);
                std::process::exit(2);
            }
        }
    } else {
        usage();
        std::process::exit(1);
    }
}
