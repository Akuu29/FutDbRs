use actix_web::{App, HttpServer, Responder, get, middleware};
use dotenv::dotenv;
use std::env;
use listenfd::ListenFd;

mod fut_api;
mod request;
mod mime_type;

#[get("/hello")]
async fn hello_world() -> impl Responder {
    format!("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .wrap(middleware::Logger::default()) // logger
    });

    // logger初期化
    env_logger::init();

    server = match listenfd.take_tcp_listener(0)? {
        // systenfdによってwatchしていた場合、そのhostとportを使用
        Some(listener) => server.listen(listener)?,
        // .envから読み取った環境変数使用
        None => {
            let host = env::var("HOST").expect("HOST is not found");
            let port = env::var("PORT").expect("PORT is not found");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}