use actix_web::{web, get, Responder, HttpResponse, HttpRequest, delete};
use serde::{Serialize, Deserialize};
use crate::db::postgres::generate_client;

#[derive(Serialize, Deserialize)]
pub enum Usergroup {
    USER,
    TRUSTED,
    SUPPORTER,
    STAFF,
    DEVELOPER
}

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
    followers: Vec<String>,
    usergroup: u32
}

#[derive(Serialize)]
pub struct PublicAccount {
    username: String,
    oauth_type: String,
    avatar_url: String,
    display_name: String,
    public_id: String,
    followers: Vec<String>,
    usergroup: u32
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
                    followers,
                    usergroup
                FROM
                    users
                WHERE
                    public_id='{}';
            ", path.into_inner().0).as_str(), &[]).await?;
        
            match user_query.get(0).is_some() {
                true => {
                    let public_user = PublicAccount {
                        display_name: user_query[0].get::<usize, String>(0),
                        username: user_query[0].get::<usize, String>(1),
                        avatar_url: user_query[0].get::<usize, String>(2),
                        public_id: user_query[0].get::<usize, String>(3),
                        oauth_type: user_query[0].get::<usize, String>(4),
                        followers: user_query[0].get::<usize, Vec<String>>(5),
                        usergroup: user_query[0].get::<usize, u32>(6)
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
                followers: query[0].get(9),
                usergroup: query[0].get::<usize, u32>(10)
            };
            Ok(HttpResponse::Ok().json(&user))
        },
        _ => {
            Ok(HttpResponse::Unauthorized().body("Invalid token"))
        }
    }
}

// Removes all user data and associated data (events, topics, etc.)
#[delete("/delete")]
async fn delete_user_data(_req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    let postgres = generate_client().await?;
    let validation = validate_user_token(&postgres, &token.to_string()).await?;

    if !validation {
        return Ok(HttpResponse::Unauthorized().body("Token not found"));
    }

    let user_id_query = postgres
        .query(format!("
            SELECT
                public_id
            FROM
                users
            WHERE
                token='{}';
        ", token).as_str(), &[])
        .await?;

    let user_id = user_id_query[0].get::<usize, String>(0);
    if user_id.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Token couldn't find user ID"));
    }

    // TODO: Remove user from DB, Remove user from existing events

    Ok(HttpResponse::Ok().body("Deleted user"))
}

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_personal_info)
            .service(get_public_info)
    );
}
