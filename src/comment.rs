//! # 댓글 관련한 작업을 처리하는데 사용되는 모듈
//!
//! `comment`는 코드뮤니티에서 댓글 관련 기능 처리를 위한
//! 메서드들로 구성되어 있다.

use std::{env, path::Path};

use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::{params, OptsBuilder, Pool, Result, SslOpts};
use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Deserialize, Serialize)]
pub struct Comment {
    /// 댓글의 고유 ID이다.
    pub comment_id: u32,
    /// 게시글의 고유 ID이다.
    pub post_id: u32,
    /// 사용자의 고유 ID이다.
    pub user_id: String,
    /// 사용자의 이름이다.
    pub user_name: String,
    /// 댓글의 내용이다.
    pub data: String,
    /// 댓글 작성 날짜 및 시간이다.
    pub create_at: String,
}

impl Comment {
    /// 새로운 댓글을 생성하는 메서드
    ///
    /// `comment_id`, `post_id`, `user_id`를 입력받아서 댓글 객체를 생성한다.
    /// 생성된 댓글 객체는 DB에 등록과 같은 동작이 가능하다.
    pub fn new(
        comment_id: Option<u32>,
        post_id: u32,
        user_id: String,
        data: String,
        create_at: Option<String>,
    ) -> Self {
        Self {
            comment_id: comment_id.unwrap_or(0),
            post_id,
            user_id: user_id.clone(),
            user_name: User::get_user(user_id).expect("Unknown User").user_name,
            data,
            create_at: create_at.unwrap_or("".to_string()),
        }
    }

    pub fn get(post_id: u32) -> Vec<Self> {
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
        conn.query_map(
            format!(
                "select * from comment where post_id = {} order by comment_id desc",
                post_id
            ),
            |(comment_id, post_id, user_id, data, create_at)| {
                Self::new(comment_id, post_id, user_id, data, create_at)
            },
        )
        .unwrap()
    }
    /// 댓글 객체를 DB에 삽입하는 메서드이다.
    ///
    /// Sql명령이 정상적으로 작동되지 않은 경우에 예외 처리를 할 수 있도록
    /// `Result<()>`로 값을 반환한다.
    /// # 예제
    /// ```
    /// use code_mmunity_server::comment::Comment;
    /// let new_comment = Comment::new(
    ///     0,
    ///    "unique_id_for_post".to_string(),
    /// );
    /// new_comment.insert_db().expect("Sql작업 중 문제가 발생하였습니다.")
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
            r"insert into comment(post_id, user_id, data)
        values(:post_id, :user_id, :data)",
            params! {
                "post_id" => self.post_id,
                "user_id" => self.user_id,
                "data" => self.data,
            },
        )
    }
}

/// JSON 을 통해 새로 등록해야 할 댓글을 받을 때 필요한 구조체이다.
#[derive(Deserialize, Serialize)]
pub struct CommentRequest {
    post_id: u32,
    user_id: String,
    data: String,
}

#[get("/api/comments/{post_id}")]
pub async fn get_comment_api(post_id: web::Path<u32>) -> impl Responder {
    println!("GET /api/comments");
    let result = Comment::get(post_id.clone());
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json;charset=utf-8"))
        .json(result)
}

#[post("/api/comments")]
pub async fn insert_comment_api(request: Json<CommentRequest>) -> impl Responder {
    println!("POST /api/comments");
    let new_comment = Comment::new(
        None,
        request.post_id,
        request.user_id.clone(),
        request.data.clone(),
        None,
    );
    match new_comment.insert_db() {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
