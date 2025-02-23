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

#[derive(Deserialize)]
pub struct PostDeleteRequest {
    token: String,
    id: String
}

#[derive(Deserialize)]
pub struct PostSearchRequest {
    search_text: String,
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

    dbg!(&info.token);
    dbg!(&token_result);

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

pub async fn delete(info: web::Json<PostDeleteRequest>) -> impl Responder {
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

    if let Some(post) = post {
        // Authenticate
        let token_result = validate_jwt(&info.token);
        if let Ok(token) = token_result {
            if token.id == post.author_id {
                // Delete the post.
                let delete_post = prisma
                    .post()
                    .delete(post::id::equals(info.id.to_string()))
                    .exec()
                    .await;
                
                if delete_post.is_err() {
                    return HttpResponse::InternalServerError().body("Internal Server Error")
                };

            } else {
                return HttpResponse::Forbidden().body("Forbidden");
            }
        } else {
            return HttpResponse::Forbidden().body("Invalid token");
        }
    }

    HttpResponse::Ok().body("")
}

pub async fn search(info: web::Json<PostSearchRequest>) -> impl Responder {
    // Create Prisma Client
    let prisma = match PrismaClient::_builder().build().await {
        Ok(prisma) => prisma,
        Err(err) => {
            eprintln!("Failed to build Prisma Client: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        },
    };

    // Trim the search text
    let search_text: String = info.search_text.trim().to_string();

    // Find posts
    let posts = prisma
        .post()
        .find_many(vec![post::content::contains(search_text.clone())])
        .exec()
        .await;
    
    match posts {
        Ok(ref posts) => posts,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    HttpResponse::Ok().json(posts)
}
