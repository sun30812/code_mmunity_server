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
#[derive(Deserialize, Serialize)]
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
    /// 생성자를 통해 포스트 생성하는 예제
    /// ```
    /// use code_mmunity_server::post::Post;
    /// let new_post = Post::new(
    ///    "unique_id_for_user".to_string(),
    ///    "Post Title".to_string(),
    ///    "rust".to_string(),
    ///    "Rust is awsome".to_string(),
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
    /// `user_name`의 경우 `User`테이블을 통해 가져오도록 처리한다. DB에서 받아온 각 속성을 받아서 처리하는 메서드이기 때문에
    /// 해당 메서드를 사용하므로써 DB에 접속되지 않는다.
    ///
    /// # 예제
    /// Sql 쿼리를 통해 단일 포스트를 가져오는 예제
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
    ///
    /// # Panics
    ///
    /// `Post`의 `user_id`가 유효한 사용자 고유 ID가 아닌 경우 패닉이 발생한다.
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
    /// DB에 존재하는 모든 포스트를 반환하는 메서드이다.
    ///
    /// DB에 모든 포스트를 요청하는 질의문을 수행 후 반환된 값 들을 `Vec<Post>`형태로 반환한다.
    /// # 예제
    /// 포스트들을 최신순으로 가져오는 예시
    /// ```
    /// let posts = Post::get_posts(PostOrder::Recent);
    /// for post in &posts {
    ///     println!("요청한 포스트의 제목은 {}이며, 작성자는 {} 입니다.", post.title, post.user_name);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// 해당 메서드는 아래와 같은 경우 패닉이 발생한다.
    /// - DB접속에 필요한 환경변수가 주어지지 않은 경우
    /// - DB에 접속이 제한시간을 초과한 경우
    /// - DB 서버 접속에 SSL을 사용하는데 인증서 파일이 존재하지 않는 경우
    pub fn get_posts() -> Vec<Self> {
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
        conn
        .query_map(
            "select post_id, user_id, title, language, substr(data, 1, 35), likes, report_count, create_at from post order by post_id desc",
            |(post_id, user_id, title, language, data, likes, report_count, create_at)| Post::from_db(post_id, user_id, title, language, data, likes, report_count, create_at)
        )
        .unwrap()
    }
    /// `post_id`를 받아서 DB에서 단일 포스트를 찾아 반환하는 메서드이다.
    ///
    /// 찾고자 하는 포스트가 존재하는 경우와 그렇지 않은 경우의 예외 처리를 할 수 있도록
    /// `Option<Post>`로 값을 반환한다.
    /// # 예제
    /// ```
    /// let post = Post::get_post(post_id);
    /// match post {
    ///     Some(result) => println!("요청한 포스트의 제목은 {}이며, 작성자는 {} 입니다.", result.title, result.user_name),
    ///     None => println!("요청하신 포스트를 찾을 수 없습니다.")
    /// }
    /// ```
    /// # Panics
    ///
    /// 해당 메서드는 아래와 같은 경우 패닉이 발생한다.
    /// - DB접속에 필요한 환경변수가 주어지지 않은 경우
    /// - DB에 접속이 제한시간을 초과한 경우
    /// - DB 서버 접속에 SSL을 사용하는데 인증서 파일이 존재하지 않는 경우
    pub fn get_post(post_id: web::Path<String>) -> Option<Self> {
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
        conn.query_first(format!("select * from post where post_id={}", post_id))
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
            )
    }
    /// 포스트 객체를 DB에 삽입하는 메서드이다.
    ///
    /// Sql명령이 정상적으로 작동되지 않은 경우에 예외 처리를 할 수 있도록
    /// `Result<()>`로 값을 반환한다.
    /// # 예제
    /// ```
    /// use code_mmunity_server::post::Post;
    /// let new_post = Post::new(
    ///    "unique_id_for_user".to_string(),
    ///    "Post Title".to_string(),
    ///    "rust".to_string(),
    ///    "Rust is awsome".to_string(),
    /// );
    /// new_post.insert_post().expect("Sql작업 중 문제가 발생하였습니다.")
    /// ```
    /// # Panics
    ///
    /// 해당 메서드는 아래와 같은 경우 패닉이 발생한다.
    /// - DB접속에 필요한 환경변수가 주어지지 않은 경우
    /// - DB에 접속이 제한시간을 초과한 경우
    /// - DB 서버 접속에 SSL을 사용하는데 인증서 파일이 존재하지 않는 경우
    pub fn insert_db(self) -> Result<()> {
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
        conn.exec_drop(
            r"insert into post(user_id, title, language, data, likes, report_count)
        values(:user_id, :title, :language, :data, :likes, :report_count)",
            params! {
                "user_id" => self.user_id,
                "title" => self.title,
                "language" => self.language,
                "data" => self.data,
                "likes" => self.likes,
                "report_count" => self.report_count,
            },
        )
    }
    /// 포스트 객체를 DB에서 제거하는 메서드이다.
    ///
    /// Sql명령이 정상적으로 작동되지 않은 경우에 예외 처리를 할 수 있도록
    /// `Result<()>`로 값을 반환한다.
    /// # 예제
    /// ```
    /// use code_mmunity_server::post::Post;
    /// let new_post = Post::new(
    ///    "unique_id_for_user".to_string(),
    ///    "Post Title".to_string(),
    ///    "rust".to_string(),
    ///    "Rust is awsome".to_string(),
    /// );
    /// let trash_post_request = DeletePostRequest { user_id: "unique_user_id".to_string(), post_id: "unique_post_id".to_string() };
    /// Post::delete_post(trash_post_request).expect("작업 중 문제가 발생하였습니다.")
    /// ```
    /// # Panics
    ///
    /// 해당 메서드는 아래와 같은 경우 패닉이 발생한다.
    /// - DB접속에 필요한 환경변수가 주어지지 않은 경우
    /// - DB에 접속이 제한시간을 초과한 경우
    /// - DB 서버 접속에 SSL을 사용하는데 인증서 파일이 존재하지 않는 경우
    pub fn delete_post(request: web::Query<DeletePostRequest>) -> Result<()> {
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
        conn.exec_drop(
            "delete from post where user_id = :user_id and post_id = :post_id",
            params! {
                "user_id" => request.user_id.clone(),
                "post_id" => request.post_id.clone(),
            },
        )
    }
}

/// JSON 을 통해 새로 등록해야 할 포스트를 받을 때 필요한 구조체이다.
#[derive(Deserialize, Serialize)]
pub struct PostRequest {
    user_id: String,
    title: String,
    language: String,
    data: String,
}
/// JSON 을 통해 삭제해야 할 포스트를 받을 때 필요한 구조체이다.
#[derive(Deserialize)]
pub struct DeletePostRequest {
    /// 포스트를 작성한 유저의 실제 구분 ID이다.
    pub user_id: String,
    /// 포스트의 고유 ID이다.
    pub post_id: String,
}

#[get("/api/posts")]
pub async fn get_posts_api() -> impl Responder {
    println!("GET /api/posts");
    let results = Post::get_posts();
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json;charset=utf-8"))
        .json(results)
}

#[get("/api/posts/{post_id}")]
pub async fn get_post_api(post_id: web::Path<String>) -> impl Responder {
    println!("GET /api/posts with ID");
    let result = Post::get_post(post_id);
    match result {
        Some(result) => HttpResponse::Ok()
            .insert_header(("Content-Type", "application/json;charset=utf-8"))
            .json(result),
        None => HttpResponse::NotFound()
            .insert_header(("Content-Type", "application/text;charset=utf-8"))
            .body("요청한 post_id는 존재하지 않는 포스트 입니다."),
    }
}

#[post("/api/posts")]
pub async fn insert_post_api(request: Json<PostRequest>) -> impl Responder {
    println!("POST /api/posts");
    let new_post = Post::new(
        request.user_id.clone(),
        request.title.clone(),
        request.language.clone(),
        request.data.clone(),
    );
    match new_post.insert_db() {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[delete("/api/posts")]
pub async fn delete_post_api(request: web::Query<DeletePostRequest>) -> impl Responder {
    println!("DELETE /api/posts");
    match Post::delete_post(request) {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
