use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod prisma;
mod jwt;
mod accounts;
mod posts;
mod replies;

async fn index() -> impl Responder {
    "DeThread Backend API v1.0 - Written in Actix"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/user")
                    .route("/add", web::post().to(accounts::add_user))
                    .route("/login", web::post().to(accounts::login))
            )
            .route("/post", web::post().to(posts::create_post))
            .route("/post", web::get().to(posts::get_post))
            .route("/post", web::delete().to(posts::delete))
            .route("/post/search", web::get().to(posts::search))
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

