use std::{env, process, collections::HashMap};
use actix_web::{web::{self, Redirect}, get, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use rand::prelude::*;
use crate::db::postgres::generate_client;

#[derive(Deserialize)]
struct DiscordAuthRes {
    code: String,
}

#[derive(Serialize)]
struct DiscordInitialReq {
    client_id: String,
    client_secret: String,
    grant_type: String,
    code: String,
    redirect_uri: String,
}

#[derive(Deserialize, Debug)]
struct DiscordInitialRes {
    access_token: String,
    token_type: String,
}

#[derive(Deserialize, Debug)]
struct DiscordUserRes {
    id: String,
    username: String,
    avatar: String,
    email: String
}

#[derive(Serialize, Deserialize)]
struct GithubAuthRes {
    code: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubInitialRes {
    access_token: String,
    scope: String,
    token_type: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubUserRes {
    login: String,
    avatar_url: String,
    id: u64 
}

#[derive(Deserialize, Serialize, Debug)]
struct GithubUserEmail {
    email: String,
    primary: bool
}

async fn fetch_discord(data: DiscordInitialReq, endpoint: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut initial_mapped = HashMap::new();
    initial_mapped.insert("client_id", data.client_id);
    initial_mapped.insert("client_secret", data.client_secret);
    initial_mapped.insert("code", data.code);
    initial_mapped.insert("grant_type", data.grant_type);
    initial_mapped.insert("redirect_uri", data.redirect_uri);

    let client = reqwest::Client::new();
    let initial_res = client.post(format!("{}/oauth2/token", endpoint))
        .form(&initial_mapped)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    let initial_parsed: DiscordInitialRes = serde_json::from_str(initial_res.text().await?.as_str())?;

    let user_res = client.get("https://discord.com/api/users/@me")
        .header("authorization", format!("{} {}", initial_parsed.token_type, initial_parsed.access_token))
        .send()
        .await?;

    let user_parsed: DiscordUserRes = serde_json::from_str(user_res.text().await?.as_str())?;

    let postgres_client = generate_client().await?;
    let query = postgres_client.query(format!("
        SELECT * FROM users WHERE public_id='{}'; 
    ", user_parsed.id).as_str(), &[]).await?;

    let token: String; 
    match query.get(0).is_some() {
        true => {
            // User does exist
            postgres_client.batch_execute(format!("
                UPDATE users 
                    SET 
                        avatar_url='https://cdn.discordapp.com/avatars/{}/{}', 
                        username='{}' 
                    WHERE 
                        public_id='{}';
            ", user_parsed.id, user_parsed.avatar, user_parsed.username, user_parsed.id).as_str()).await?;
        
            let token_query = postgres_client
                .query(format!("
                    SELECT token FROM users WHERE public_id='{}';
                ", user_parsed.id).as_str(), &[])
                .await?;

            token = token_query[0].get(0);
        },
        _ => {
            // User doesn't exist
            let mut rng = rand::thread_rng();
            let random_num: f64 = rng.gen(); 

            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", user_parsed.email, random_num * 999_999_999f64).into_bytes());

            token = format!("{:X}", hasher.finalize());
            
            postgres_client.batch_execute(format!("
                INSERT INTO users (
                    token, 
                    display_name, 
                    email, 
                    username, 
                    avatar_url, 
                    oauth_type, 
                    public_id, 
                    joined_groups, 
                    joined_events,
                    followers
                )
                VALUES (
                    '{}', 
                    '{}', 
                    '{}', 
                    '{}', 
                    'https://cdn.discordapp.com/avatars/{}/{}', 
                    'discord', 
                    '{}', 
                    '{{}}', 
                    '{{}}',
                    '{{}}'
                );
            ", token, user_parsed.username, user_parsed.email, user_parsed.username, user_parsed.id, user_parsed.avatar, user_parsed.id).as_str()).await?;
        }
    }

    Ok(token)
}

#[get("/discord")]
async fn auth_discord(req: web::Query<DiscordAuthRes>, _req: HttpRequest) -> impl Responder {
    let req = req.into_inner();    

    let endpoint: String = "https://discord.com/api/v10".to_string();
    let id: String = env::var("DISCORD_OAUTH_ID").unwrap_or_else(|e| {
        println!("We broke...\n{e}");
        process::exit(1);
    });
    let secret: String = env::var("DISCORD_OAUTH_SECRET").unwrap_or_else(|e| {
        println!("We broke...\n{e}");
        process::exit(1);
    });
    let redirect: String = "http://127.0.0.1:3001/api/auth/discord".to_string();
    let code = req.code;

    let data = DiscordInitialReq {
        client_id: id,
        client_secret: secret,
        grant_type: "authorization_code".to_string(),
        code,
        redirect_uri: redirect
    };     

    let token = fetch_discord(data, &endpoint).await.unwrap_or_else(|e| {
        println!("Broke\n{}", e);
        process::exit(1)
    });

    let frontend_url: String = env::var("FRONTEND_URL").unwrap_or_else(|e| {
        println!("{}", e);
        "http://localhost:3000".to_string()
    });

    let mut login_url_finish: String = String::from(frontend_url);
    login_url_finish.push_str(format!("/login?token={}", token).as_str());

    Redirect::to(login_url_finish).see_other()
}

async fn fetch_github(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let client_id = env::var("GITHUB_OAUTH_ID").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });
    let client_secret = env::var("GITHUB_OAUTH_SECRET").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    let initial_res = client.post("https://github.com/login/oauth/access_token")
        .form(&[("code", &code), ("client_id", &client_id), ("client_secret", &client_secret)])
        .header("accept", "application/json")
        .send()
        .await?;

    let parsed: GithubInitialRes = serde_json::from_str(initial_res.text().await?.as_str())?;
    let access_token = parsed.access_token; 

    let user_res = client.get("https://api.github.com/user")
        .header("authorization", format!("{} {}", parsed.token_type, access_token).as_str())
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "request")
        .send()
        .await?;
  
    let user_parsed: GithubUserRes = serde_json::from_str(user_res.text().await?.as_str())?;

    let email_res = client.get("https://api.github.com/user/emails")
        .header("authorization", format!("{} {}", parsed.token_type, access_token).as_str())
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "request")
        .send()
        .await?;

    let email_parsed: Vec<GithubUserEmail> = serde_json::from_str(email_res.text().await?.as_str())?;
    let mut email: String = "".to_string();
    for e in email_parsed.into_iter() {
        if e.primary {
            email = e.email;
            break;
        }
    }

    let postgres = generate_client().await?;
    let query = postgres.query(format!("
        SELECT
            *
        FROM
            users
        WHERE
            public_id='{}';
    ", user_parsed.id.to_string()).as_str(), &[]).await?;

    let token: String; 
    match query.get(0).is_some() {
        true => {
            // User does exist
            postgres.batch_execute(format!("
                UPDATE users 
                    SET 
                        avatar_url='{}',
                        username='{}' 
                    WHERE 
                        public_id='{}';
            ", user_parsed.avatar_url, user_parsed.login, user_parsed.id.to_string()).as_str()).await?;
        
            let token_query = postgres 
                .query(format!("
                    SELECT token FROM users WHERE public_id='{}';
                ", user_parsed.id.to_string()).as_str(), &[])
                .await?;

            token = token_query[0].get(0);
        },
        _ => {
            // User doesn't exist
            let mut rng = rand::thread_rng();
            let random_num: f64 = rng.gen(); 

            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}{}", email, random_num * 999_999_999f64, user_parsed.id).into_bytes());

            token = format!("{:X}", hasher.finalize());
            
            postgres.batch_execute(format!("
                INSERT INTO users (
                    token, 
                    display_name, 
                    email, 
                    username, 
                    avatar_url, 
                    oauth_type, 
                    public_id, 
                    joined_groups, 
                    joined_events,
                    followers
                )
                VALUES (
                    '{}', 
                    '{}', 
                    '{}', 
                    '{}', 
                    '{}',
                    'github', 
                    '{}', 
                    '{{}}', 
                    '{{}}',
                    '{{}}'
                );
            ", token, user_parsed.login, email, user_parsed.login, user_parsed.avatar_url, user_parsed.id.to_string()).as_str()).await?;
        }
    }


    Ok(token)
}

#[get("/github")]
async fn auth_github(req: web::Query<GithubAuthRes>, _req: HttpRequest) -> impl Responder {
    let req = req.into_inner();
    let token = fetch_github(req.code).await.unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });
    let frontend_url: String = env::var("FRONTEND_URL").unwrap_or_else(|e| {
        println!("{}", e);
        "http://localhost:3000".to_string()
    });

    let mut login_url_finish: String = String::from(frontend_url);
    login_url_finish.push_str(format!("/login?token={}", token).as_str());

    Redirect::to(login_url_finish).see_other()
}

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth_discord)
            .service(auth_github)
    );
} 
