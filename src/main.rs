use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::env;
use std::net::Ipv4Addr;

mod likes;
mod post;

/// 서버의 시작점이다.
///
/// `APP_PORT` 환경변수를 지정하면 포트 번호 변경이 가능하다.
/// 포트 번호를 지정하지 않을 시 포트번호는 8080번으로 지정되어있다.
/// `addr`을 통해 IP주소를 직접 전달하거나 LOCALHOST등으로 설정이 가능하다.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = Ipv4Addr::UNSPECIFIED;
    let port = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(value) => value
            .parse()
            .expect("FUNCTIONS_CUSTOMHANDLER_PORT가 숫자가 아닙니다."),
        Err(_) => 3000,
    };
    println!("서버가 작동됩니다.");
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(post::get_posts)
            .service(post::get_post)
            .service(likes::modify_likes)
            .service(post::new)
    })
    .bind((addr, port))?
    .run()
    .await
}
