use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::RedisSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time, Key, SameSite},
    dev::Service,
    http::{
        self,
        header::{self, HeaderValue},
    },
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use app_config::parse_config;
use bhw_types::{actix_web_validator::JsonConfig, validation::ValidationError};
use data::cv::CVManager;
use db::init_db;
use middleware::csrf::Csrf;
use routes::{
    auth::{check_signed_in_route, sign_in_route, sign_out_route},
    cv::{cv_exists_route, delete_cv_route, get_cv_download_url_route, upload_cv_route},
    group::{
        create_group_route, get_my_group_route, join_group_route, leave_group_route,
        rename_my_group_route,
    },
    healthz::healthz_route,
    password_reset::{forgot_password_pin_route, forgot_password_route},
    profile::{get_profile_route, update_profile_route},
    sign_up::{account_activate_route, sign_up_route},
};
use util::validation::humanise_validation_error;

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
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .filter_module("sqlx::query", log::LevelFilter::Warn)
            .init();

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

            let json_config = JsonConfig::default().limit(4096).error_handler(|err, _| {
                match err {
                    bhw_types::actix_web_validator::Error::JsonPayloadError(e) => {
                        ValidationError::Parse(e.to_string())
                    }
                    bhw_types::actix_web_validator::Error::Validate(e) => {
                        ValidationError::Values(humanise_validation_error(&e))
                    }
                    _ => ValidationError::Unknown,
                }
                .into()
            });

            let cv_manager = CVManager::client(&config);

            let multipart_form_config = MultipartFormConfig::default().total_limit(10000000);

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
                .wrap_fn(|req, srv| {
                    let fut = srv.call(req);
                    async {
                        let mut res = fut.await?;
                        let h = res.headers_mut();

                        h.append(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
                        h.append(
                            header::CONTENT_SECURITY_POLICY,
                            HeaderValue::from_static("frame-ancestors 'none'"),
                        );
                        h.append(
                            header::X_CONTENT_TYPE_OPTIONS,
                            HeaderValue::from_static("nosniff"),
                        );
                        h.append(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

                        Ok(res)
                    }
                })
                .app_data(json_config)
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(db_con.clone()))
                .app_data(web::Data::new(cv_manager))
                .app_data(web::Data::new(multipart_form_config))
                .service(healthz_route)
                .service(sign_up_route)
                .service(account_activate_route)
                .service(check_signed_in_route)
                .service(sign_in_route)
                .service(sign_out_route)
                .service(forgot_password_route)
                .service(forgot_password_pin_route)
                .service(get_profile_route)
                .service(update_profile_route)
                .service(create_group_route)
                .service(rename_my_group_route)
                .service(join_group_route)
                .service(get_my_group_route)
                .service(leave_group_route)
                .service(upload_cv_route)
                .service(cv_exists_route)
                .service(get_cv_download_url_route)
                .service(delete_cv_route)
        }
    };

    #[cfg(feature = "ldap")]
    {
        use crate::ldap::{check_pending_users, connect_ldap, Ldap, PendingUserCheckError};
        use log::{error, warn};

        let ldap_task_config = config.clone();

        tokio::task::spawn(async move {
            let db_con = init_db(&ldap_task_config.clone()).await;
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            let mut ldap: Option<Ldap> = None;
            loop {
                interval.tick().await;
                let unwrapped_ldap = match ldap {
                    None => {
                        ldap = connect_ldap(ldap_task_config.clone())
                            .await
                            .inspect_err(|e| error!("ldap reinit {}", e))
                            .ok();
                        match ldap {
                            None => {
                                continue;
                            }
                            Some(ref v) => v,
                        }
                    }
                    Some(ref v) => v,
                };
                let r = check_pending_users(unwrapped_ldap.clone(), db_con.clone()).await;
                match r {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("error in ldap loop {}", e);
                        match e {
                            PendingUserCheckError::DBError(e) => {
                                warn!("{}", e);
                            }
                            PendingUserCheckError::LdapError(e) => {
                                warn!("{}", e);
                                ldap = None
                            }
                        }
                    }
                }
            }
        });
    }

    HttpServer::new(http_factory)
        .bind((config.bind_address, config.bind_port))?
        .run()
        .await
}
