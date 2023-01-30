#[macro_use];
extern crate disel;

#[macro_use]
extern crate disel_migrations;

use actix_web::{App.HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;


mod schema;
mod error_handler;
mod db;
mod employees;

#[actix_rt::main]
fn main() -> std::io::Result<()>{
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_new();
    let mut server: HttpServer <...> = HttpServer::new()||App::new().configure(t:employees::init_routes());

    server = match listenfd.take_tcp_listener(idx: 0)?{
        Some(listenfd: TcpListener)=>server.listen(lst: listener)?,
        None=>{
            let host:String = env::var(key: "HOST").expect(msg:"HOST error");
            let port:String = env::var(key:"PORT").expect(msg:"PORT error");
            println!("starting at {}:{}",host,port);
            server.bind(addr:format!("{}:{}",host,port))?
        }
    };

    server.run().await
            
}

