use actix_web::{middleware::Logger, web, App, HttpServer};
use app_config::parse_config;
use db::init_db;
use routes::sign_up::sign_up_route;

mod app_config;
mod schema;
mod db;
mod models;
mod routes;
mod util;
mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = parse_config();

    let http_factory = {
        let config = config.clone();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let db_con = init_db(config.clone());

        move || App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(config.to_owned()))
            .app_data(web::Data::new(db_con.to_owned()))
            .service(sign_up_route)
    };

    HttpServer::new(http_factory)
        .bind((config.bind_address, config.bind_port))?
        .run()
        .await
}

