// Gets config from a file in the same working directory as the program called
// bot.cfg, and allows async read only access to the file
// returns a BotConfig type variable
use std::io::prelude::*;
use tokio_postgres::{NoTls, Error, Row};
use toml;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use tokio::fs;
use tokio::prelude::*;
use tokio::fs::File;
use std::path::Path;

pub struct BotConfig{
    discord_api : String,
    detailed_network : bool,
    detailed_debug : bool,
    banned_words_global : Vector<String>,
    banned_links_global : Vector<String>
}
impl BotConfig {
    pub fn get_discord_api(&self) -> &String{
        &self.discord_api
    }
    pub async fn is_network(&self)->&bool{
        &self.detailed_network
    }
    pub async fn is_debug(&self)->&bool{
        &self.detailed_debug
    }
    pub async fn contains_word(&self, word : &str) -> &bool{
        if &self.banned_words_global.contains(word){
            return &true;
        }
        else{
            return &false;
        }
        &false
    }
    pub async fn contains_link(&self, link : &str) -> &bool{
        if &self.banned_links_global.contains(link){
            return &true;
        }
        else{
            return &false;
        }
        &false
    }

}



#[derive(Deserialize)]
struct Discord{
    discord_api : String,
}

#[derive(Deserialize)]
struct Preferences{
    detailed_network : bool,
    detailed_debug : bool,
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct Globals{
    banned_search : Vec<String>,
    banned_links : Vec<String>,
}


#[derive(Serialize)]
#[derive(Deserialize)]
struct Config{
    discord : Discord,
    preferences : Preferences,
    globals : Globals,
}

// File write example
/*
title="bot config file"

# hashtags indicate line comments
# go to https://discord.com/developers/applications and make a bot
# don't share this key with anyone but yourself
# replace <API_KEY> with the api key you got from discord, and enclose them in ""
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
    let mut generated_file = File::create(file_name).await?;
    generated_file.write_all(r#"
title="bot config file"

# hashtags indicate line comments
# go to https://discord.com/developers/applications and make a bot
# don't share this key with anyone but yourself
# replace <API_KEY> with the api key you got from discord, and enclose them in ""
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
banned_links=[]"#.as_ref())
}

// NOTE:                        if file isnt found => Create empty bot.toml file
//       if it is invalid or some other file error => panic!
pub async fn get_config() -> Result<BotConfig, ()>{
    let mut cfg_content = fs::read_to_string("bot.toml").await?;

    // note: this will panic if bot.toml is wrong
    let bot_config : Config = toml::from_str(cfg_content).unwrap();
    let return_cfg : BotConfig = BotConfig{
        discord_api : bot_config.discord.discord_api,
        detailed_network : bot_config.preferences.detailed_network,
        detailed_debug : bot_config.preferences.detailed_debug,
        banned_links_global : bot_config.globals.banned_links,
        banned_words_global : bot_config.globals.banned_search,
    };
    Ok(return_cfg)
}

pub async fn generate_bot_toml() -> Result<(),&str>{
    let who_is_rem = Path::new("bot.toml").exists();
    if who_is_rem{
        return Err("Error: File already exists in FileSystem!");
    }
    else{
        if let Err(why) = generate_config_file("bot.toml").await{
            eprintln!("Error while creating bot.toml: {:?}", why);
            return Err(why);
        }
    }
    Ok(())
}