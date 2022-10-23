use std::env;
use std::path::Path;

use actix_web::{post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
}

impl User {
    pub fn get_user(user_id: String) -> Option<Self> {
        let ssl = match env::var("USE_SSL") {
            Ok(value) => {
                if value == "true" {
                    Some(SslOpts::default().with_root_cert_path(Some(Path::new(
                        "./cert/DigiCertGlobalRootCA.crt.pem",
                    ))))
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(
                env::var("DB_SERVER").expect("DB_SERVER가 설정되지 않음"),
            ))
            .tcp_port(
                env::var("DB_PORT")
                    .expect("DB_PORT가 설정되지 않음")
                    .parse::<u16>()
                    .expect("DB_PORT가 올바른 형식이 아님"),
            )
            .user(Some(env::var("DB_USER").expect("DB_USER가 설정되지 않음")))
            .pass(Some(
                env::var("DB_PASSWD").expect("DB_PASSWD가 설정되지 않음"),
            ))
            .db_name(Some(
                env::var("DB_DATABASE").expect("DB_DATABASE가 설정되지 않음"),
            ))
            .ssl_opts(ssl);
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let result = conn
            .query_first(format!("select * from user where user_id='{}'", user_id))
            .unwrap()
            .map(|(user_id, user_name)| User { user_id, user_name });
        result
    }
}

#[post("/api/users")]
pub async fn new_user(new_user: web::Query<User>) -> impl Responder {
    println!("POST /api/users");
    let ssl =
        match env::var("USE_SSL") {
            Ok(value) => {
                if value == "true" {
                    Some(SslOpts::default().with_root_cert_path(Some(Path::new(
                        "./cert/DigiCertGlobalRootCA.crt.pem",
                    ))))
                } else {
                    None
                }
            }
            Err(_) => None,
        };
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(
            env::var("DB_SERVER").expect("DB_SERVER가 설정되지 않음"),
        ))
        .tcp_port(
            env::var("DB_PORT")
                .expect("DB_PORT가 설정되지 않음")
                .parse::<u16>()
                .expect("DB_PORT가 올바른 형식이 아님"),
        )
        .user(Some(env::var("DB_USER").expect("DB_USER가 설정되지 않음")))
        .pass(Some(
            env::var("DB_PASSWD").expect("DB_PASSWD가 설정되지 않음"),
        ))
        .db_name(Some(
            env::var("DB_DATABASE").expect("DB_DATABASE가 설정되지 않음"),
        ))
        .ssl_opts(ssl);
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn.exec_drop(
        r"replace into user
        values(:user_id, :user_name)",
        params! {
            "user_id" => new_user.user_id.clone(),
            "user_name" => new_user.user_name.clone()
        },
    )
    .unwrap();
    HttpResponse::Created()
}
