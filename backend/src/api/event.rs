use actix_web::{web, Responder, HttpResponse, HttpRequest};
use crate::db::postgres::generate_client;
use super::user::validate_user_token;

#[derive(Serialize, Deserialize)]
struct 

async fn is_apart_of_event(postgres: tokio_postgres::Client, id: String, event_id: String) -> Result<bool, Box<dyn std::error::Error>> {
    let event_query = postgres
        .query(format!("
            SELECT
                allowed_users
            FROM 
                events
            WHERE
                id='{}';
        ", event_id).as_str(), &[])
        .await?;

    if event_query.get(0).is_none() {
        false
    }

    let user_query = postgres
        .query(format!("
            SELECT
                public_id
            FROM
                users
            WHERE
                public_id='{}';
        ", user_id).as_str(), &[])
        .await?;

    if user_query.get(0).is_none() {
        false
    }

    for i in event_query[0].get(0).len() {
        if event_query[0].get(0)[i] == user_id {
            true
        }
    }

    false
}

async fn is_event_private(postgres: tokio_postgres::Client, event_id: String) -> Result<bool, Box<dyn std::error::Error>> {
    let event_query = postgres
        .query(format!("
            SELECT
                private
            FROM 
                events
            WHERE
                id='{}';
        ", event_id).as_str(), &[])
        .await?;

    if event_query.get(0).is_none() {
        false
    }

    Ok(event_query[0].get(0) == true)
}

#[get("/{event_id}")]
async fn get_event_details(_req: HttpRequest, req: web::Query<>) -> impl Responder {
    let postgres = generate_client().await?
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    let 
    let validation: bool = validate_user_token(&postgres, token).await?;

    match validation {
        true => {
            let apart = is_apart_of_event(postgres, id, event_id) 
        },
        _ => {
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

pub fn config_events(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/event")
            .service(get_event_details)
    );
}
