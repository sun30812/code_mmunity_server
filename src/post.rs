//! # Post
//!
//! `post`는 코드뮤니티에서 포스트 객체 관련 작업을 처리하기 위한
//! 요소 및 메서드들로 이루어져 있다.
//!
//! `post`를 통해 포스트 목록 요청을 받을 수 있고, 포스트를 받았을 때 처리 방식도
//! 이곳에서 수행한다.

use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

/// 코드뮤니티에 쓰이는 포스트 객체이다.
///
/// 실제로 포스트를 생성하려면 생성자인 `new()`를 대신 사용해야한다.
/// # 예제
/// ```
/// let my_post = Post::new("user_name", "this is my post");
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    /// 포스트의 고유 ID 이다. DB에서 auto_increment에 의해 값이 자동으로 증가한다.
    pub id: i64,
    /// 포스트를 작성한 유저의 실제 구분 ID이다.
    pub uid: String,
    /// 포스트의 제목이다.
    pub title: String,
    /// 포스트를 작성한 유저의 ID(보여지는 ID) 이다.
    pub user_name: String,
    /// 포스트 내용이다.
    pub data: String,
    /// 포스트의 공감 수 이다.
    pub likes: i64,
    /// 포스트가 신고당한 횟수이다.
    pub report_count: i64,
    /// 포스트가 생성된 날짜이다.
    pub create_at: String,
}

impl Post {
    pub fn new(uid: String, title: String, user: String, data: String) -> Self {
        Self {
            id: 0,
            uid,
            title,
            user_name: user,
            data,
            likes: 0,
            report_count: 0,
            create_at: "2022-10-11 21:29:30".to_string(),
        }
    }
}

/// JSON 을 통해 새로 등롥해야 할 포스트를 받을 때 필요한 구조체이다.
///
///
#[derive(Deserialize, Serialize, Debug)]
pub struct PostRequest {
    uid: String,
    title: String,
    user_name: String,
    data: String,
}

/// 포스트에 대해 GET 요청을 받는 경우의 동작을 정의한 메서드이다.
///
#[get("/api/posts")]
pub async fn get_posts(post_id: String) -> impl Responder {
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
    let mut results = conn
        .query_map(
            "select * from post",
            |(id, uid, title, user_name, data, likes, report_count, create_at)| Post {
                id,
                uid,
                title,
                user_name,
                data,
                likes,
                report_count,
                create_at,
            },
        )
        .unwrap();
    let test = Post::new(
        "gn$ikjshk34343".to_string(),
        "test".to_string(),
        "테스트".to_string(),
        "hello, db".to_string(),
    );
    results.push(test);
    println!("{:?}", results);
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json;charset=utf-8"))
        .json(results)
}

#[post("/api/posts")]
pub async fn new(request: Json<PostRequest>) -> impl Responder {
    let new_post = Post::new(
        request.uid.to_string(),
        request.title.to_string(),
        request.user_name.to_string(),
        request.data.to_string(),
    );
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
        r"insert into post(uid, title, user_name, data, likes, report_count)
        values(:uid, :title, :user_name, :data, :likes, :report_count)",
        params! {
            "uid" => new_post.uid,
            "title" => new_post.title,
            "user_name" => new_post.user_name,
            "data" => new_post.data,
            "likes" => new_post.likes,
            "report_count" => new_post.report_count,
        },
    )
    .unwrap();

    HttpResponse::Created()
}
