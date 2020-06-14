// Gets config from a file in the same working directory as the program called
// bot.cfg, and allows async read only access to the file
// returns a BotConfig type variable
use std::fs::File;
use std::io::prelude::*;
use tokio_postgres::{NoTls, Error, Row};
use toml;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;


pub struct BotConfig{
    discord_api : String,
    detailed_network : bool,
    detailed_debug : bool,
    banned_words : Vector<String>,
    banned_links : Vector<String>
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

// File write example
/*
title="bot config file"

# hashtags indicate line comments
# go to https://discord.com/developers/applications and make a bot
# don't share this key with anyone but yourself
[discord]
discord_api=<API_KEY>

[preferences]
# true  => Network Congestion, Download/Upload Speed, Packet Loss, Ping
# false => ping
detailed_network=true
# true  => Program RAM usage, CPU usage, Threads
detailed_debug=true

[globals]
# banned search terms
# lists go like ["term1","term2"]ㅋ
banned_search=[]
# note: use links
# this is good: ["https://www.youtube.com/watch?v=dQw4w9WgXcQ", "https://www.youtube.com/watch?v=F5oQoNMpqi8"]
# this is no good: ["F5oQoNMpqi8", "dQw4w9WgXcQ", "hentai"]
banned_links=[]
 */

pub async fn generate_config_file(file_name: &str) ->std::io::Result<()>{
    let mut generated_file = File::create(file_name)?;
    generated_file.write_all(r#"
title="bot config file"

# hashtags indicate line comments
# go to https://discord.com/developers/applications and make a bot
# don't share this key with anyone but yourself
# replace <API_KEY> with the api key you got from discord
[discord]
discord_api=<API_KEY>

[preferences]
# true  => Network Congestion, Download/Upload Speed, Packet Loss, Ping
# false => ping
detailed_network=true
# true  => Program RAM usage, CPU usage, Threads
detailed_debug=true

[globals]
# banned search terms
# lists go like ["term1", "term2"]
banned_search=[]
# note: use links
# this is good: ["https://www.youtube.com/watch?v=dQw4w9WgXcQ", "https://www.youtube.com/watch?v=F5oQoNMpqi8"]
# this is no good: ["F5oQoNMpqi8", "dQw4w9WgXcQ", "hentai"]
banned_links=[]"#.as_ref())
}

// NOTE:                        if file isnt found => Create empty bot.toml file
//       if it is invalid or some other file error => panic!
pub fn get_config() -> Option<BotConfig>{
    let toml_to_string = match File::open("bot.toml"){
        Ok(s) => s,
        Err(why)=> {
            eprintln!("Error: No bot.toml file found. It has been generated for you, please fill it out!");
            let mut create_file = match File::create("bot.toml"){
                Ok(a) => a,
                Err(why) => {
                    panic!("Error! Could not create bot.toml!");
                }
            };


        }

    };
}
