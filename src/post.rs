//! # Post
//!
//! `post`는 코드뮤니티에서 포스트 객체 관련 작업을 처리하기 위한
//! 요소 및 메서드들로 이루어져 있다.
//!
//! `post`를 통해 포스트 목록 요청을 받을 수 있고, 포스트를 받았을 때 처리 방식도
//! 이곳에서 수행한다.

use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 코드뮤니티에 쓰이는 포스트 객체이다.
///
/// 실제로 포스트를 생성하려면 생성자인 `new()`를 대신 사용해야한다.
/// # 예제
/// ```
/// let my_post = Post::new("user", "this is my post");
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    /// 포스트의 고유 ID 입니다. 생성자에 의해 값이 정해진다.
    pub id: String,
    /// 포스트의 제목이다.
    pub title: String,
    /// 포스트를 작성한 유저의 ID 이다.
    pub user: String,
    /// 포스트 내용이다.
    pub data: String,
    /// 포스트의 공감 수 이다.
    pub likes: i32,
    /// 포스트가 신고당한 횟수이다.
    pub report_count: i32,
    /// 포스트가 삭제되기 전 남은 일 이다.
    pub left_days: i32,
}

impl Post {
    pub fn new(title: String, user: String, data: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            user,
            data,
            likes: 0,
            report_count: 0,
            left_days: 0,
        }
    }
}

/// JSON 을 통해 포스트 작성 정보를 받을 때 필요한 구조체이다.
///
///
#[derive(Deserialize, Serialize, Debug)]
pub struct PostRequest {
    user: String,
    data: String,
}

/// 포스트에 대해 GET 요청을 받는 경우의 동작을 정의한 메서드이다.
///
#[get("/posts")]
pub async fn get_posts() -> impl Responder {
    let mut results = vec![];
    let test = Post::new(
        "test".to_string(),
        "sn30".to_string(),
        "hello, db".to_string(),
    );
    results.push(test);
    results.push(Post::new(
        "test".to_string(),
        "user2".to_string(),
        "I like Windows OS".to_string(),
    ));
    results.push(Post::new(
        "test".to_string(),
        "user2".to_string(),
        "I like Windows OS".to_string(),
    ));
    results.push(Post::new(
        "test".to_string(),
        "user2".to_string(),
        "fn main() {\n  println!(\"232323\");\n}".to_string(),
    ));

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(results)
}

#[post("/posts")]
pub async fn new(request: Json<PostRequest>) -> impl Responder {
    let new_post = Post::new(
        "test".to_string(),
        request.user.to_string(),
        request.data.to_string(),
    );
    println!("{:?}", new_post);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(new_post)
}
