//! A Http server returns Hello World String as Response.

use xitca_web::{App, handler::handler_service, route::get};

async fn hi() -> &'static str {
    "Hello world"
}

fn main() -> std::io::Result<()> {
    App::new()
        .at("/", get(handler_service(hi)))
        .serve()
        .bind("127.0.0.1:8080")?
        .run()
        .wait()
}
