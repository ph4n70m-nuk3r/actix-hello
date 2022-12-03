extern crate core;

mod config;
mod router;

use crate::config::*;
use crate::router::*;

use std::env;

use actix_web::{web, App, HttpServer};

/* Application Entry Point */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = get_args().await;
    let toml_file = &args[args.len() - 1];
    let config = read_toml_config(toml_file).await;
    match config {
        Ok(config) => {
            print_info_messages(&config).await;
            return start_server(&config).await;
        }
        Err(e) => {
            panic!("[FATAL] {}", e)
        }
    }
}

async fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => panic!("Program was run without a command line."),
        1 => panic!("This application expects 1 argument (path to config file) but got 0."),
        2 => println!("[DEBUG] Detected 1 argument."),
        _ => {
            panic!("This application expects 1 argument (path to config file) but got more than 1.")
        }
    }
    return args;
}

async fn print_info_messages(config: &Config) {
    println!(
        "[INFO]  Service Short Description: {}",
        config.metadata.description.short
    );
    println!(
        "[INFO]  Service Long Description:  {}",
        config.metadata.description.long
    );
    println!(
        "[INFO]  Actix-Web Server Port:     {}",
        config.spec.server.port
    );
}

async fn start_server(config: &Config) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hi", web::get().to(hi)) /* Explicit HTTP GET */
            .service(hello) /* Annotated GET */
            .service(hello_name) /* Annotated GET with PATH PARAM */
            .service(echo) /* Annotated POST */
    })
    .bind(("127.0.0.1", config.spec.server.port))?
    .run()
    .await
}
