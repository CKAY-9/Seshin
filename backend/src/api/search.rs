use actix_web::{web, get, HttpRequest, Responder, HttpResponse};

use crate::db::postgres::generate_client;
use crate::api::user::validate_user_token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    search_query: String
} 

#[derive(Serialize, Deserialize)]
struct UserPreview {
    display_name: String,
    username: String,
    avatar_url: String,
    public_id: String
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    users: Vec<UserPreview>
}

#[get("/all")]
async fn get_nonspecific_search_data(_req: HttpRequest, req: web::Query<SearchQuery>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let postgres = generate_client().await?;
    let search_query = req.into_inner().search_query;
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();

    let validation: bool = validate_user_token(&postgres, &token.to_string()).await?;
    match validation {
        true => {
            let user_query = postgres.query(format!("
                SELECT
                    display_name,
                    username,
                    avatar_url,
                    public_id
                FROM
                    users
                WHERE
                    username ILIKE '%{}%'
                LIMIT
                    10;
            ", search_query).as_str(), &[]).await?;

            let mut users: Vec<UserPreview> = vec![];
            for i in 0..user_query.len() {
                users.push(UserPreview { 
                    display_name: user_query[i].get(0), 
                    username: user_query[i].get(1), 
                    avatar_url: user_query[i].get(2),
                    public_id: user_query[i].get(3)
                });
            }

            let search_json = SearchResponse {
                users 
            };
            Ok(HttpResponse::Ok().json(&search_json))
        },
        _ => Ok(HttpResponse::Unauthorized().body("Token invalid"))
    }
}

pub fn config_search(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/search")
            .service(get_nonspecific_search_data)
    );
}
