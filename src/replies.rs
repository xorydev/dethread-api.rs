use actix_web::{web, HttpResponse, Responder};
use crate::prisma::PrismaClient;
use serde::{Deserialize, Serialize};
use crate::jwt::{validate_jwt, Claims};
use crate::prisma::{post, account, reply};

#[derive(Deserialize, Serialize)]
pub struct ReplyRequest {
    token: String,
    post: String,
    content: String
}

#[derive(Deserialize, Serialize)]
pub struct ReplyResponse { id: String }


pub async fn reply(info: web::Json<ReplyRequest>) -> impl Responder {
    // Create Prisma Client
    let prisma = PrismaClient::_builder().build().await.unwrap();

    // Validate JWT
    let jwt_result = validate_jwt(&info.token);
    if let Ok(jwt_claims) = jwt_result {
        // Locate the target post
        let post_id = info.post.clone();
        let post_result = prisma.post().find_unique(post::id::equals(post_id)).exec().await;
        if let Ok(_) = post_result {
            // Create the reply
            let author = jwt_claims.id;
            let post = info.post.clone();
            let reply_content = info.content.clone();
            let reply_result = prisma.reply().create(
                "reply_meow".to_string(),
                reply_content,
                crate::prisma::account::UniqueWhereParam::IdEquals(author),
                crate::prisma::post::UniqueWhereParam::IdEquals(post),
                vec![]
            ).exec().await;
            
            // Send the response

            if let Ok(reply) = reply_result {
                return HttpResponse::Ok().json(reply);
            } else {
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
            
        } else {
            return HttpResponse::NotFound().body("Post not found");
        }
    } else {
        return HttpResponse::Forbidden().body("Invalid token");
    }
    
}
