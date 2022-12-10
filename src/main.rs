use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use log::{info, log_enabled, Level};
use sysinfo::{System, SystemExt};

#[get("/api")]
async fn api_respone() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = sys.host_name();
    let total_memory = sys.total_memory();

    log::info!("Connect to Test");

    // Return the system information in a JSON response
    HttpResponse::Ok().json((hostname, total_memory))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(api_respone))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/*async fn dashboard() -> HttpResponse {
    let mut sys = System::new();
}*/
