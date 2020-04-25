#![warn(clippy::all)]

mod api;
mod auth;
mod in_memory_db;
mod web_server;

fn main() {
    use clap::{crate_authors, crate_description, crate_name, crate_version, App};

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    println!("{} v{}", crate_name!(), crate_version!());

    let listen_addr = "0.0.0.0";
    let listen_port = "8081";
    let bind = format!("{}:{}", listen_addr, listen_port);

    // Run Tide server
    web_server::run(bind);
}
