use std::collections::HashMap;

use actix_web::{web, Responder, HttpResponse, get, HttpRequest, post};
use crate::db::postgres::generate_client;
use super::user::validate_user_token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EventFetchQuery {
    event_id: String
} 

#[derive(Deserialize)]
struct EventCreationData {
    name: String,
    time: usize,
    description: String,
    private: bool,
    organizers: Vec<String>,
    members: Vec<String>
}

async fn is_apart_of_event(postgres: &tokio_postgres::Client, id: &String, event_id: &String) -> Result<bool, Box<dyn std::error::Error>> {
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

    for row in user_query {
        if row.get::<usize, String>(0) == id.to_owned() {
            return Ok(true);
        } 
    }

    Ok(false)
}

async fn is_event_private(postgres: &tokio_postgres::Client, event_id: &String) -> Result<bool, Box<dyn std::error::Error>> {
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
        return Ok(false);
    };

    Ok(event_query[0].get::<usize, bool>(0) == true)
}

#[get("/{event_id}")]
async fn get_event_details(_req: HttpRequest, req: web::Query<EventFetchQuery>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let req = req.into_inner();
    let postgres = generate_client().await?;
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    let event_id = req.event_id;
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
            let id = id_fetch[0].get::<usize, String>(0);
            let apart = is_apart_of_event(&postgres, &id, &event_id).await?;
            let private = is_event_private(&postgres, &event_id).await?;

            match private {
                true => {
                    if !apart {
                        return Ok(HttpResponse::Unauthorized().body("No apart of event!"));
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

#[post("/create")]
async fn create_new_event(_req: HttpRequest, req: web::Json<EventCreationData>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let req = req.into_inner();
    
    let token = _req.headers().get("authorization").unwrap().to_str().unwrap();
    if token.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Token not found!"));
    }

    let postgres = generate_client().await?;
    let user_query = postgres
        .query(format!("
            SELECT
                public_id
            FROM
                users
            WHERE
                token='{}'
        ", token).as_str(), &[])
        .await?;
    if user_query.get(0).is_none() {
        return Ok(HttpResponse::BadRequest().body("Couldn't fetch public ID!"));
    }

    let id_md5_hash = md5::compute(format!("{}{}{}", req.time, req.organizers.get(0).unwrap(), req.name).into_bytes());
    let id = format!("{:x}", id_md5_hash);

    postgres.batch_execute(format!("
        INSERT INTO events (
            id,
            name,
            description,
            organizers,
            members,
            private,
            time
        )
        VALUES (
            '{}',
            '{}',
            '{}',
            '[{}]',
            '[{}]',
            '{}',
            '{}'
        )
    ", id, req.name, req.description, user_query[0].get::<usize, String>(0), user_query[0].get::<usize, String>(0), req.private, req.time).as_str());

    let mut response = HashMap::new();
    response.insert("event_id", id);

    Ok(HttpResponse::Ok().json(&response))
}

pub fn config_events(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/event")
            .service(get_event_details)
    );
}
