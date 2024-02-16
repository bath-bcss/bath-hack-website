use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{time, Key},
    middleware::Logger,
    web, App, HttpServer,
};
use app_config::parse_config;
use db::init_db;
use routes::sign_up::sign_up_route;

mod app_config;
mod data;
mod db;
mod models;
mod routes;
mod schema;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = parse_config();

    let http_factory = {
        let config = config.clone();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let db_con = init_db(config.clone());

        move || {
            App::new()
                .wrap(Logger::default())
                .wrap(
                    SessionMiddleware::builder(
                        RedisActorSessionStore::new(config.redis_string.clone()),
                        Key::try_from(config.cookie_secret.as_bytes())
                            .expect("Parsing cookie secret"),
                    )
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(time::Duration::hours(1)),
                    )
                    .build(),
                )
                .app_data(web::Data::new(config.to_owned()))
                .app_data(web::Data::new(db_con.to_owned()))
                .service(sign_up_route)
        }
    };

    HttpServer::new(http_factory)
        .bind((config.bind_address, config.bind_port))?
        .run()
        .await
}
