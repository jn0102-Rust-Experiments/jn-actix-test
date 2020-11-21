extern crate jn_actix_initializer_macros;
extern crate jn_actix_initializer;
// extern crate regex;

mod mod1;
mod model;

use actix_web::{middleware, web::ServiceConfig, App, HttpServer};
use jn_actix_initializer_macros::ServiceConfigInitializer;
use jn_actix_initializer::ServiceConfigInitializer;
use mod1::*;
use std::env;
// use regex::Regex;

#[derive(ServiceConfigInitializer)]
struct Cfg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("localhost".to_string());
    let listen_port = env::var("LISTEN_PORT").unwrap_or("80".to_string());
    let binding = format!("{}:{}", listen_addr, listen_port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(Cfg::register_handlers)
    })
    .bind(binding)?
    .run()
    .await
}
