//! # 포스트(게시물) 관련 동작을 정의하는 모듈
//!
//! `post`는 코드뮤니티에서 포스트 객체 관련 작업을 처리하기 위한
//! 요소 및 메서드들로 이루어져 있다.
//!
//! `post`를 통해 포스트 목록 요청을 받을 수 있고, 포스트를 받았을 때 처리 방식도
//! 이곳에서 수행한다.

use crate::user::User;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

/// 코드뮤니티에 쓰이는 포스트 객체이다.
///
/// 실제로 새 포스트를 생성하려면 생성자인 `new()`를 대신 사용해야한다.  
/// 만일 DB에서 포스트를 받아오는 경우 `from_db()`를 사용하면 된다.
#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    /// 포스트의 고유 ID 이다. DB에서 auto_increment에 의해 값이 자동으로 증가한다.
    pub post_id: u64,
    /// 포스트를 작성한 유저의 실제 구분 ID이다.
    pub user_id: String,
    /// 포스트의 제목이다.
    pub title: String,
    /// 포스트를 작성한 유저의 이름이다.
    pub user_name: String,
    /// 포스트에 작성된 프로그래밍 언어 종류이다.
    pub language: String,
    /// 포스트 내용이다.
    pub data: String,
    /// 포스트의 공감 수 이다.
    pub likes: u64,
    /// 포스트가 신고당한 횟수이다.
    pub report_count: u64,
    /// 포스트가 생성된 날짜이다.
    pub create_at: String,
}

impl Post {
    /// 새 포스트를 생성할 때 사용하는 생성자이다.
    ///
    /// 작성한 새 포스트를 만들 때 사용되므로 이전 DB에 존재하는 포스트를 가져올 때는 생성자를 사용하면 안된다.
    /// `user_id`에는 포스트 작성자의 이름이, `title`에는 포스트의 제목이, `language`에는 포스트 본문에 사용된
    /// 프로그래밍 언어를 작성해야 한다. 본문은 `data`에 해당한다.
    ///
    /// # 예제
    /// request를 통해 포스트에 필요한 몇몇 정보를 가져온 경우 포스트 생성하는 예제
    /// ```
    /// use code_mmunity_server::post::Post;
    /// let new_post = Post::new(
    ///    request.user_id.clone(),
    ///    request.title.clone(),
    ///    request.language.clone(),
    ///    request.data.clone(),
    /// );
    /// ```
    pub fn new(user_id: String, title: String, language: String, data: String) -> Self {
        Self {
            post_id: 0,
            user_id: user_id.clone(),
            title,
            language,
            user_name: User::get_user(user_id).expect("Unknown User").user_name,
            data,
            likes: 0,
            report_count: 0,
            create_at: "2022-10-11 21:29:30".to_string(),
        }
    }
    /// DB에서 포스트를 가져올 때 사용하는 메서드이다.
    ///
    /// DB에 존재하는 포스트를 가져올 때 사용되므로 `Post`테이블에 존재하는 모든 속성을 인자로 사용한다.
    /// `user_name`의 경우 `User`테이블을 통해 가져오도록 처리한다.
    ///
    /// # 예제
    /// DB에 쿼리를 통해 단일 포스트를 가져오는 예제
    /// ```
    /// let result = conn
    /// .query_first(format!("select * from post where post_id={}", post_id))
    /// .unwrap()
    /// .map(
    ///     |(post_id, user_id, title, language, data, likes, report_count, create_at)| {
    ///          Post::from_db(
    ///              post_id,
    ///              user_id,
    ///              title,
    ///              language,
    ///              data,
    ///              likes,
    ///              report_count,
    ///              create_at,
    ///          )
    ///      },
    ///  );
    /// ```
    pub fn from_db(
        post_id: u64,
        user_id: String,
        title: String,
        language: String,
        data: String,
        likes: u64,
        report_count: u64,
        create_at: String,
    ) -> Self {
        Self {
            post_id,
            user_id: user_id.clone(),
            title,
            language,
            user_name: User::get_user(user_id).expect("Unknown User").user_name,
            data,
            likes,
            report_count,
            create_at,
        }
    }
}

/// JSON 을 통해 새로 등록해야 할 포스트를 받을 때 필요한 구조체이다.
#[derive(Deserialize, Serialize, Debug)]
pub struct PostRequest {
    user_id: String,
    title: String,
    language: String,
    data: String,
}
/// JSON 을 통해 삭제해야 할 포스트를 받을 때 필요한 구조체이다.
#[derive(Deserialize)]
pub struct DeletePostRequest {
    user_id: String,
    post_id: String,
}

/// 포스트에 대해 GET 요청을 받는 경우의 전체 포스트를 전달하는 동작을 정의한 메서드이다.
///
#[get("/api/posts")]
pub async fn get_posts() -> impl Responder {
    println!("GET /api/posts");
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
    let results = conn
        .query_map(
            "select post_id, user_id, title, language, substr(data, 1, 35), likes, report_count, create_at from post",
            |(post_id, user_id, title, language, data, likes, report_count, create_at)| Post::from_db(post_id, user_id, title, language, data, likes, report_count, create_at)
        )
        .unwrap();
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json;charset=utf-8"))
        .json(results)
}

/// 단일 포스트에 대해 GET 요청을 받는 경우의 단일 포스트를 반환하는 동작을 정의한 메서드이다.
///
#[get("/api/posts/{post_id}")]
pub async fn get_post(post_id: web::Path<String>) -> impl Responder {
    println!("GET /api/posts with ID");
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
    let result = conn
        .query_first(format!("select * from post where post_id={}", post_id))
        .unwrap()
        .map(
            |(post_id, user_id, title, language, data, likes, report_count, create_at)| {
                Post::from_db(
                    post_id,
                    user_id,
                    title,
                    language,
                    data,
                    likes,
                    report_count,
                    create_at,
                )
            },
        );
    match result {
        Some(result) => HttpResponse::Ok()
            .insert_header(("Content-Type", "application/json;charset=utf-8"))
            .json(result),
        None => HttpResponse::NotFound()
            .insert_header(("Content-Type", "application/text;charset=utf-8"))
            .body("요청한 post_id는 존재하지 않는 포스트 입니다."),
    }
}
/// 새 포스트를 생성한 후 Post요청을 한 경우 처리방식을 기술한 메서드이다.
#[post("/api/posts")]
pub async fn make_post(request: Json<PostRequest>) -> impl Responder {
    println!("POST /api/posts");
    let new_post = Post::new(
        request.user_id.clone(),
        request.title.clone(),
        request.language.clone(),
        request.data.clone(),
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
    match conn.exec_drop(
        r"insert into post(user_id, title, language, data, likes, report_count)
        values(:user_id, :title, :language, :data, :likes, :report_count)",
        params! {
            "user_id" => new_post.user_id,
            "title" => new_post.title,
            "language" => new_post.language,
            "data" => new_post.data,
            "likes" => new_post.likes,
            "report_count" => new_post.report_count,
        },
    ) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    // .unwrap();
    // HttpResponse::Created()
}
/// 포스트 삭제 요청을 한 경우 처리방식을 기술한 메서드이다.
#[delete("/api/posts")]
pub async fn delete_post(request: web::Query<DeletePostRequest>) -> impl Responder {
    println!("DELETE /api/posts");
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
    match conn.exec_drop(
        "delete from post where user_id = :user_id and post_id = :post_id",
        params! {
            "user_id" => request.user_id.clone(),
            "post_id" => request.post_id.clone(),
        },
    ) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    // .unwrap();
    // HttpResponse::Created()
}
