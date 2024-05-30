use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod prisma;
mod jwt;
mod accounts;
mod posts;

async fn index() -> impl Responder {
    "DeThread Backend API v1.0 - Written in Actix"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/")
                    .route("/user/add", web::post().to(accounts::add_user))
                    .route("/user/login", web::post().to(accounts::login))
                    .route("/post/", web::post().to(posts::create_post))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

