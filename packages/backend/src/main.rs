use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::RedisActorSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{Key, time},
    middleware::Logger,
    web, App, HttpServer,
};
use app_config::parse_config;
use db::{init_db, DbPool};
use routes::{sign_up::{account_activate_route, sign_up_route}, auth::{check_signed_in_route, sign_in_route}};

mod app_config;
mod data;
mod db;
mod models;
mod routes;
mod schema;
mod util;
mod ldap;

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
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .cookie_http_only(false)
                    .build(),
                )
                .app_data(web::Data::new(config.to_owned()))
                .app_data(web::Data::<DbPool>::new(db_con.to_owned()))
                .service(sign_up_route)
                .service(account_activate_route)
                .service(check_signed_in_route)
                .service(sign_in_route)
        }
    };

    HttpServer::new(http_factory)
        .bind((config.bind_address, config.bind_port))?
        .run()
        .await
}
