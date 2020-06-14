// Gets config from a file in the same working directory as the program called
// bot.cfg, and allows async read only access to the file
// returns a BotConfig type variable
use std::fs::File;
use std::io::prelude::*;
use tokio_postgres::{NoTls, Error, Row};
use toml;
use serde_derive::{Deserialize, Serialize};


pub struct BotConfig{
    discord_api : String,
}
impl BotConfig {
    pub fn get_discord_api(&self) -> &String{
        &self.discord_api
    }
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct

pub async fn get_query(host : &str, usr : &str, query : &str, param : &str) -> Result<Vec<Row>, Error>{
    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(format!("host={} user={}",host,usr).as_str(), NoTls).await?;

    // Spawn the tokio thread since the connection does its own thing and talks to the DB
    tokio::spawn(async move {
        if let Err(e) = connection.await{
            eprintln!("PostgreSQL Connection Error: {}", e);
        }
    });

    let rows = client.query(query, &[&param]).await?;

    Ok(rows)
}

// NOTE:                        if file isnt found => Create empty bot.toml file
//       if it is invalid or some other file error => panic!
pub fn get_config() -> Option<BotConfig>{

}
