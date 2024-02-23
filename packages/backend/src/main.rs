use actix_cors::Cors;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::RedisSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time, Key, SameSite},
    http,
    middleware::Logger,
    web, App, HttpServer,
};
use app_config::parse_config;
use db::init_db;
use middleware::csrf::Csrf;
use routes::{
    auth::{check_signed_in_route, sign_in_route, sign_out_route},
    profile::{get_profile_route, update_profile_route},
    sign_up::{account_activate_route, sign_up_route},
};

mod app_config;
mod data;
mod db;
mod ldap;
mod middleware;
mod models;
mod routes;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = parse_config();

    let http_factory = {
        let config = config.clone();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let db_con = init_db(&config).await;
        let store = RedisSessionStore::new(&config.redis_string)
            .await
            .expect("initialising redis session store");

        move || {
            let cors = Cors::default()
                .allowed_origin(config.clone().allowed_origin.as_str())
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::CONTENT_TYPE])
                .supports_credentials()
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
                        store.clone(),
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
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(db_con.clone()))
                .service(sign_up_route)
                .service(account_activate_route)
                .service(check_signed_in_route)
                .service(sign_in_route)
                .service(sign_out_route)
                .service(get_profile_route)
                .service(update_profile_route)
        }
    };

    HttpServer::new(http_factory)
        .bind((config.bind_address, config.bind_port))?
        .run()
        .await
}
