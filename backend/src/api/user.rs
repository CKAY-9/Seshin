use actix_web::{web, get, Responder, HttpResponse, HttpRequest};
use serde::Serialize;
use crate::db::postgres::generate_client;

#[derive(Serialize)]
pub struct FullAccount {
    token: String,
    email: String,
    username: String,
    avatar_url: String,
    oauth_type: String,
    public_id: String,
    joined_groups: Vec<String>,
    joined_events: Vec<String>,
    display_name: String,
    followers: Vec<String>
}

#[derive(Serialize)]
pub struct PublicAccount {
    username: String,
    oauth_type: String,
    avatar_url: String,
    display_name: String,
    public_id: String,
    followers: Vec<String>
}

pub async fn validate_user_token(postgres_client: &tokio_postgres::Client, token: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let query = postgres_client.query(format!("
        SELECT
            token
        FROM 
            users
        WHERE
            token='{}';
    ", token).as_str(), &[]).await?;

    Ok(query.get(0).is_some())
}

#[get("/{public_id}")]
async fn get_public_info(_req: HttpRequest, path: web::Path<(String,)>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let postgres = generate_client().await?;
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();

    let validation: bool = validate_user_token(&postgres, &token.to_string()).await?;
    match validation {
        true => {
            let user_query = postgres.query(format!("
                SELECT
                    display_name,
                    username,
                    avatar_url,
                    public_id,
                    oauth_type,
                    followers
                FROM
                    users
                WHERE
                    public_id='{}';
            ", path.into_inner().0).as_str(), &[]).await?;
        
            match user_query.get(0).is_some() {
                true => {
                    let public_user = PublicAccount {
                        display_name: user_query[0].get(0),
                        username: user_query[0].get(1),
                        avatar_url: user_query[0].get(2),
                        public_id: user_query[0].get(3),
                        oauth_type: user_query[0].get(4),
                        followers: user_query[0].get(5)
                    };

                    Ok(HttpResponse::Ok().json(&public_user))
                },
                _ => Ok(HttpResponse::NotFound().body("User not found"))
            }
        },
        _ => Ok(HttpResponse::Unauthorized().body("Invalid token"))
    }
}

#[get("/me")]
async fn get_personal_info(_req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let postgres = generate_client().await?;
   
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    let query = postgres.query(format!("
        SELECT
            *
        FROM
            users
        WHERE
            token='{}';
    ", token).as_str(), &[]).await?;

    match query.get(0).is_some() {
        true => {
            let user = FullAccount {
                token: query[0].get(0),
                display_name: query[0].get(1),
                email: query[0].get(2),
                username: query[0].get(3),
                avatar_url: query[0].get(4),
                oauth_type: query[0].get(5),
                public_id: query[0].get(6),
                joined_groups: query[0].get(7),
                joined_events: query[0].get(8),
                followers: query[0].get(9) 
            };
            Ok(HttpResponse::Ok().json(&user))
        },
        _ => {
            Ok(HttpResponse::Unauthorized().body("Invalid token"))
        }
    }
}

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_personal_info)
            .service(get_public_info)
    );
}
