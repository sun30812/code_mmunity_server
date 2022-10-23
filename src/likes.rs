//! # Likes
//!
//! `likes`는 코드뮤니티에서 공감 관련 기능 처리를 위한
//! 메서드들로 구성되어 있다.

use actix_web::{patch, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Deserialize)]
pub enum LikeMode {
    Increment,
    Decrement,
}

#[derive(Deserialize)]
pub struct LikeRequest {
    post_id: i64,
    mode: LikeMode,
}

/// 공감 관련 작업을 요청받았을 때 수행하는 동작
#[patch("/api/likes")]
pub async fn modify_likes(info: web::Query<LikeRequest>) -> impl Responder {
    println!("PATCH /api/likes");
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

    let result = match info.mode {
        LikeMode::Increment => {
            match conn.exec_drop(
                r"update post
        set likes = likes + 1
        where post_id = :post_id",
                params! {
                    "post_id" => info.post_id.clone()
                },
            ) {
                Ok(_) => HttpResponse::Created()
                    .insert_header(("Content-Type", "application/text;charset=utf-8;"))
                    .body("Increment likes"),
                Err(error) => HttpResponse::BadRequest()
                    .insert_header(("Content-Type", "application/text;charset=utf-8;"))
                    .body(error.to_string()),
            }
        }
        LikeMode::Decrement => {
            match conn.exec_drop(
                r"update post
        set likes = likes - 1
        where post_id = :post_id",
                params! {
                    "post_id" => info.post_id.clone()
                },
            ) {
                Ok(_) => HttpResponse::Created()
                    .insert_header(("Content-Type", "application/text;charset=utf-8;"))
                    .body("Decrement likes"),
                Err(error) => HttpResponse::BadRequest()
                    .insert_header(("Content-Type", "application/text;charset=utf-8;"))
                    .body(error.to_string()),
            }
        }
    };
    result
}
