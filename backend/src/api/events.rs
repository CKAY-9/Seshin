use actix_web::{web, Responder, HttpResponse, get, HttpRequest};
use crate::db::postgres::generate_client;
use super::user::validate_user_token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EventFetchQuery {
    event_id: String
} 

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
        return Ok(false);
    }

    let user_query = postgres
        .query(format!("
            SELECT
                public_id
            FROM
                users
            WHERE
                public_id='{}';
        ", id).as_str(), &[])
        .await?;

    if user_query.get(0).is_none() {
        return Ok(false);
    }

    for i in event_query[0].get::<usize>(0).len() {
        if event_query[0].get(0)[i] == id {
            return Ok(true);
        }
    }

    Ok(false)
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
        return Ok(());
    };

    Ok(event_query[0].get(0) == true)
}

#[get("/{event_id}")]
async fn get_event_details(_req: HttpRequest, req: web::Query<EventFetchQuery>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let postgres = generate_client().await?;
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    let event_id = req.0;
    let id_fetch = postgres
        .query(format!("
            SELECT 
                public_id
            FROM
                users
            WHERE
                token='{}';
        ", token).as_str(), &[])
        .await?;

    let validation: bool = validate_user_token(&postgres, &token.to_string()).await?;
    

    match validation {
        true => {
            let id = id_fetch[0].get(0);
            let apart = is_apart_of_event(postgres, id, event_id).await?;
            let private = is_event_private(postgres, event_id).await?;

            match private {
                true => {
                    if !apart {
                        Ok(HttpResponse::Unauthorized());
                    }

                    Ok(HttpResponse::Ok().body("User allowed"))
                },
                _ => {
                    Ok(HttpResponse::Ok().body("User allowed"))
                }
            }           
        },
        _ => {
            Ok(HttpResponse::Unauthorized().body("Invalid token"))
        }
    }
}

pub fn config_events(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/event")
            .service(get_event_details)
    );
}
