//! # 공감 관련 동작을 정의하는 모듈
//!
//! `likes`는 코드뮤니티에서 공감 관련 기능 처리를 위한
//! 메서드들로 구성되어 있다.

use actix_web::{patch, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use std::env;
use std::path::Path;

/// 공감 수를 늘릴지 줄일지 선택하는 모드이다.
#[derive(Deserialize)]
pub enum LikeMode {
    /// 공감 수 증가
    Increment,
    /// 공감 수 감소
    Decrement,
}

/// 공감 관련 작업을 요청받았을 때 필요한 구조체
///
/// `post_id`에는 공감 수를 줄이거나 늘릴 포스트의 고유 ID가 들어가고
/// `mode`에는 `LikeMode`에 따라 공감을 늘리는 요청인지 줄이는 요청인지 확인한다.
///
#[derive(Deserialize)]
pub struct LikeRequest {
    /// 포스트의 고유 ID이다.
    pub post_id: i64,
    /// 공감 수를 늘릴지 줄일지 선택하는 모드이다.
    pub mode: LikeMode,
}

impl LikeRequest {
    /// 공감 수를 조작하는 메서드
    ///
    /// `info`에는 쿼리 스트링을 통해 `LikeRequest` 구조체에 명시된 값을 받아 동작을 처리한다.
    /// 공감 수 조작 실패에 대한 예외처리를 할 수 있도록 `Result<()>`로 반환한다.
    ///
    /// # 예제
    /// ```
    /// match LikeRequest::modify_likes(info) {
    ///     Ok(_) => println!("공감 수 업데이트 됨"),
    ///     Err(error) => panic!(error)
    /// }
    /// ```
    /// # Panics
    ///
    /// 해당 메서드는 아래와 같은 경우 패닉이 발생한다.
    /// - DB접속에 필요한 환경변수가 주어지지 않은 경우
    /// - DB에 접속이 제한시간을 초과한 경우
    /// - DB 서버 접속에 SSL을 사용하는데 인증서 파일이 존재하지 않는 경우
    pub fn modify_likes(info: web::Query<LikeRequest>) -> Result<()> {
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
        match info.mode {
            LikeMode::Increment => conn.exec_drop(
                r"update post
            set likes = likes + 1
            where post_id = :post_id",
                params! {
                    "post_id" => info.post_id.clone()
                },
            ),
            LikeMode::Decrement => conn.exec_drop(
                r"update post
            set likes = likes - 1
            where post_id = :post_id",
                params! {
                    "post_id" => info.post_id.clone()
                },
            ),
        }
    }
}

#[patch("/api/likes")]
pub async fn modify_likes_api(info: web::Query<LikeRequest>) -> impl Responder {
    println!("PATCH /api/likes");
    match LikeRequest::modify_likes(info) {
        Ok(_) => HttpResponse::Created()
            .insert_header(("Content-Type", "application/text;charset=utf-8;"))
            .body("update likes"),
        Err(error) => HttpResponse::BadRequest()
            .insert_header(("Content-Type", "application/text;charset=utf-8;"))
            .body(error.to_string()),
    }
}
