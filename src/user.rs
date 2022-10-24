//! # 사용자 관련 동작을 정의하는 모듈
//!
//! `user`는 코드뮤니티에서 사용자 관리 작업을 처리하기 위한
//! 요소 및 메서드들로 이루어져 있다.
//!
//! `user`를 통해 사용자 이름을 확인하거나, 계정 탈퇴를 할 시 작업을
//! 이곳에서 수행한다.
use actix_web::{post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use std::env;
use std::path::Path;

/// 코드뮤니티에 쓰이는 사용자 객체이다.
///
/// 별도의 생성자가 없이 직접 생성해주면 된다.  
/// 만일 `user_id`를 통해 사용자 이름을 받아오는 경우 `get_user()`를 활용하면 된다.
/// # 예제
/// ```
/// let new_user = Post{"unique_id_for_user".to_string(), "sun30812".to_string()}
/// ```
#[derive(Deserialize)]
pub struct User {
    /// 사용자를 식별하는 고유 ID로 절대로 중복되서는 안된다.
    pub user_id: String,
    /// 사용자의 표시 이름이다.
    pub user_name: String,
}

impl User {
    /// `user_id`를 통해 사용자 객체를 반환하는 메서드이다.
    ///
    /// 코드뮤니티의 `post`객체는 `user_id`만 가지고 있기 때문에 작성자를 확인하기 위해서는
    /// 해당 메서드가 필요하다. 실제로 존재하는 사용자의 경우 사용자 객체를, 존재하지 않는 경우
    /// `None`을 반환하기 때문에 예외처리가 가능하다.
    /// # 예제
    /// `user_id`로 사용자의 이름을 찾아서 출력하는 예제
    /// ```
    /// let find_user = User::get_user("unique_id_for_user".to_string());
    /// match find_user {
    ///     Some(user) => println!("사용자의 이름은 {} 입니다.", user.user_name),
    ///     None => println!("존재하지 않는 사용자입니다.")
    /// }
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
/// 새로운 사용자를 등록할 때 사용되는 메서드
///
/// `new_user`에는 쿼리 스트링을 통해 `User` 구조체에 명시된 값을 받아 동작을 처리한다.
///
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
