use actix_cors::Cors;
use actix_web::{App, HttpServer};
use code_mmunity_server::comment;
use code_mmunity_server::likes;
use code_mmunity_server::post;
use code_mmunity_server::user;
use std::env;
use std::net::Ipv4Addr;

/// 서버의 시작점이다.
///
/// `APP_PORT` 환경변수를 지정하면 포트 번호 변경이 가능하다.
/// 포트 번호를 지정하지 않을 시 포트번호는 8080번으로 지정되어있다.
/// `addr`을 통해 IP주소를 직접 전달하거나 LOCALHOST등으로 설정이 가능하다.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = Ipv4Addr::UNSPECIFIED;
    let port = match env::var("APP_PORT") {
        Ok(value) => value.parse().expect("APP_PORT가 숫자가 아닙니다."),
        Err(_) => 8080,
    };
    println!("{}번 포트에서 서버가 작동됩니다.", port);
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(user::new_user_api)
            .service(post::get_posts_api)
            .service(post::get_post_api)
            .service(user::get_user_api)
            .service(user::update_user_api)
            .service(user::delete_user_api)
            .service(post::delete_post_api)
            .service(likes::modify_likes_api)
            .service(post::insert_post_api)
            .service(comment::get_comment_api)
            .service(comment::insert_comment_api)
    })
    .bind((addr, port))?
    .run()
    .await
}
