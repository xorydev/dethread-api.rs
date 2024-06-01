use actix_web::{web, Responder, HttpResponse};
use serde::Deserialize;
use crate::jwt::validate_jwt;
use crate::prisma::PrismaClient;
use crate::prisma::account;
use crate::prisma::post;
use rand::Rng;

#[derive(Deserialize)]
pub struct PostCreationRequest {
    token: String,
    title: String,
    content: String,
}

#[derive(Deserialize)]
pub struct PostGetRequest {
    id: String,
}


pub async fn create_post(info: web::Json<PostCreationRequest>) -> impl Responder {
    let prisma = match PrismaClient::_builder().build().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error building Prisma client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    // Validate the JWT token
    let token_result = validate_jwt(info.token.as_str());

    if let Ok(token) = token_result {
        let mut rng = rand::thread_rng();
        let post_id = rng.gen_range(0..999999).to_string();
        let token_id = token.id.clone(); // Clone here to avoid moving token.id

        // Find the account associated with the token
        let account = match prisma.account().find_unique(account::id::equals(token_id.clone())).exec().await {
            Ok(Some(account)) => account,
            Ok(None) => return HttpResponse::Unauthorized().body("Invalid token or account not found"),
            Err(err) => {
                eprintln!("Error querying the database: {:?}", err);
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        };

        // Create the post
        let post = match prisma.post().create(
            format!("post_{}", post_id),
            info.title.clone(), // Clone here to get a String
            info.content.clone(), // Clone here to get a String
            account::id::equals(token_id),
            vec![]
        ).exec().await {
            Ok(post) => post,
            Err(err) => {
                eprintln!("Error creating post: {:?}", err);
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        };

        return HttpResponse::Ok().json(post);
    }

    HttpResponse::Unauthorized().body("Invalid token")
}

pub async fn get_post(info: web::Json<PostGetRequest>) -> impl Responder {
    let prisma = match PrismaClient::_builder().build().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error building Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    // Search the DB for the post
    let post = prisma.post()
        .find_unique(post::id::equals(info.id.to_string()))
        .exec()
        .await
        .unwrap();

    // Return the post
    if let Some(post) = post {
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().body("Not Found")
    }

}
