use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod prisma;
use prisma::PrismaClient;
use rand::Rng;
use serde::{Deserialize, Serialize};
mod jwt;
use jwt::{create_jwt, validate_jwt};

async fn index() -> impl Responder {
    "DeThread Backend API v1.0 - Written in Actix"
}

#[derive(Deserialize)]
struct AccountCreationRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct AccountCreationResponse {
    id: String,
    token: String
}

async fn add_user(info: web::Json<AccountCreationRequest>) -> impl Responder {
    let prisma = PrismaClient::_builder().build().await.unwrap();
    let mut rng = rand::thread_rng();
    let userid: u64 = rng.gen_range(0..999999999999);
    let account: prisma::account::Data = prisma.account().create(
        format!("user_{userid}").to_string(),
        info.username.to_string(),
        info.email.to_string(),
        info.password.to_string(),
        vec![]
    ).exec().await.unwrap();   

    let response = AccountCreationResponse {
        id: account.id.clone(),
        token: create_jwt(account.id.as_str()).expect("we just generated both of these, this shouldn't fail"),
    };
    
    println!("sending response");
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/user")
                    .route("/add", web::post().to(add_user))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

