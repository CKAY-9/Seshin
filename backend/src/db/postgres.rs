use std::{env, process};

use tokio_postgres::{NoTls, Error};

pub async fn generate_client() -> Result<tokio_postgres::Client, Error> {
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    let user = env::var("POSTGRES_USER").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    let pwd = env::var("POSTGRES_PASSWORD").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });

    let port = env::var("POSTGRES_PORT").unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    let (client, connection) = 
        tokio_postgres::connect(format!("host={} user={} password={} port={}", host, user, pwd, port).as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)

}

pub async fn connect() -> Result<(), Error> {
    let client = generate_client().await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            token TEXT PRIMARY KEY,
            display_name TEXT,
            email TEXT NOT NULL,
            username TEXT NOT NULL,
            avatar_url TEXT NOT NULL,
            oauth_type TEXT NOT NULL,
            public_id TEXT NOT NULL UNIQUE,
            joined_groups TEXT[],
            joined_events TEXT[],
            followers TEXT[]
        )  
    ").await?;

    Ok(())
}
