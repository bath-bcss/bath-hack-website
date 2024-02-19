use actix_cors::Cors;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::RedisActorSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time, Key, SameSite},
    middleware::Logger,
    web, App, HttpServer,
};
use app_config::parse_config;
use db::{init_db, DbPool};
use middleware::csrf::Csrf;
use routes::{
    auth::{check_signed_in_route, sign_in_route},
    sign_up::{account_activate_route, sign_up_route},
};

mod app_config;
mod data;
mod db;
mod ldap;
mod middleware;
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
            let cors = Cors::default()
                .allowed_origin(config.clone().allowed_origin.as_str())
                .allowed_methods(vec!["GET", "POST"])
                .max_age(3600);

            let csrf = Csrf {
                allowed_origin: config.clone().allowed_origin,
            };

            App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .wrap(csrf)
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
                    .cookie_same_site(SameSite::Lax)
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
