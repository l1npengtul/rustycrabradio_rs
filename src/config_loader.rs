// Gets config from a file in the same working directory as the program called
// bot.cfg, and allows async read only access to the file
// returns a BotConfig type variable
use std::fs::File;
use std::io::prelude::*;
use tokio_postgres::{NoTls, Error, Row};

pub struct BotConfig{
    discord_api : String,
}
impl BotConfig {
    pub fn get_discord_api(&self) -> &String{
        &self.discord_api
    }
}

pub async fn get_query(host : &str, usr : &str, query : &str, param : &str) -> Result<Vec<Row>, Error>{
    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect("host="+host+" user="+usr, NoTls).await?;

    // Spawn the tokio thread since the connection does its own thing and talks to the DB
    tokio::spawn(async move {
        if let Err(e) = connection.await{
            eprintln!("PostgreSQL Connection Error: {}", e);
        }
    });

    let rows = client.query(query, &[&param]).await?;

    Ok(rows)
}

// NOTE: Using .unwrap() will cause an unrecoverable error (panic!) if the file is not found
pub fn get_config() -> BotConfig{
    let mut cfg_file = File::open("bot.cfg").unwrap();
    let mut contents = String::new();

    cfg_file.read_to_string(&mut contents);

    let mut cfg_lines = contents.lines();
    let mut discord_api : String = String::from("");

    for l in cfg_lines{
        let mut line_split = l.split("=");
        match line_split[0].as_str(){
            "DISC_API"=>discord_api = line_split[1],
        }
    }
    let bret = BotConfig{
        discord_api: discord_api,
    };
    bret
}
