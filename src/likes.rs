//! # Likes
//!
//! `likes`는 코드뮤니티에서 공감 관련 기능 처리를 위한
//! 메서드들로 구성되어 있다.

use actix_web::{post, HttpResponse, Responder};

/// 공감 추가를 요청받았을 때 수행하는 동작
#[post("/add_likes")]
pub async fn increment_likes(id: String) -> impl Responder {
    println!("Increment Likes: {}", id);
    HttpResponse::Created()
}

/// 공감 감소를 요청받았을 때 수행하는 동작
#[post("/decrement_likes")]
pub async fn decrement_likes(id: String) -> impl Responder {
    println!("Decrement Likes: {}", id);
    HttpResponse::Created()
}
