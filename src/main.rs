// actix_ping_pong/main.rs

struct Settings {
    logging: bool,
    log_level: String,
    client_url: String,
    bind_socket: String, 
}

impl Default for Settings {
    fn default() -> Self {
        return Self {
            logging: true,
            log_level: "debug".to_string(),
            client_url: "secret".to_string(),
            bind_socket: "secret".to_string(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings: Settings = Settings::default();
    let settings_data: actix_web::web::Data<Settings> = actix_web::web::Data::new(settings);

    if settings_data.logging == true {
        match settings_data.log_level.as_str() {
            "trace" => env_logger::Builder::new().filter_level(log::LevelFilter::Trace).init(),
            "debug" => env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init(),
            "info" => env_logger::Builder::new().filter_level(log::LevelFilter::Info).init(),
            "warn" => env_logger::Builder::new().filter_level(log::LevelFilter::Warn).init(),
            _ => env_logger::Builder::new().filter_level(log::LevelFilter::Error).init(),
        }
    }
    
    let socket: String = settings_data.bind_socket.clone();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
        .app_data(settings_data.clone())
        .wrap(actix_cors::Cors::default()
            .allowed_origin(settings_data.client_url.as_str())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
            .supports_credentials())
        .route("/ping", actix_web::web::get().to(ping))
    })
    .bind(socket.as_str())?
    .run()
    .await
}

async fn ping() -> impl actix_web::Responder {
    log::debug!("Sent pong");
    use serde_json::json;
    return actix_web::HttpResponse::Ok().json(json!({"value": "pong"}));
}