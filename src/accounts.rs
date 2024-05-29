use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::prisma::PrismaClient;
use crate::prisma::account;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::jwt::{create_jwt, validate_jwt};



#[derive(Deserialize)]
pub struct AccountCreationRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct AccountCreationResponse {
    id: String,
    token: String
}

#[derive(Deserialize, Serialize)]
pub struct AccountLoginRequest {
    email: String,
    password: String
}

#[derive(Deserialize, Serialize)]
struct AccountLoginResponse { token: String }

pub async fn add_user(info: web::Json<AccountCreationRequest>) -> impl Responder {
    let prisma = PrismaClient::_builder().build().await.unwrap();
    let mut rng = rand::thread_rng();
    let userid: u64 = rng.gen_range(0..999999999999);
    let account: crate::prisma::account::Data = prisma.account().create(
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
pub async fn login(info: web::Json<AccountLoginRequest>) -> Result<HttpResponse, actix_web::Error> {
    // Build the Prisma client
    let prisma = if let Ok(client) = PrismaClient::_builder().build().await {
        client
    } else {
        eprintln!("Error building Prisma client");
        return Ok(HttpResponse::InternalServerError().body("Internal Server Error"));
    };

    // Find the account in the database
    let account = if let Ok(account) = prisma.account().find_first(vec![
        account::email::equals(info.email.to_string()),
        account::password::equals(info.password.to_string())
    ]).exec().await {
        account
    } else {
        eprintln!("Error querying the database");
        return Ok(HttpResponse::InternalServerError().body("Internal Server Error"));
    };

    // Check if the account was found
    if let Some(account) = account {
        // Create the JWT
        let token = if let Ok(token) = create_jwt(account.id.as_str()) {
            token
        } else {
            eprintln!("Error creating JWT");
            return Ok(HttpResponse::InternalServerError().body("Internal Server Error"));
        };

        // Create the response
        let response = AccountLoginResponse { token };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid email or password"))
    }
}

