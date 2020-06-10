// Gets config from a file in the same working directory as the program called
// bot.cfg, and allows async read only access to the file
// returns a BotConfig type variable
use std::fs::File;
use std::io::prelude::*;

pub struct BotConfig{
    discord_api : String,
}

impl BotConfig {
    pub fn get_discord_api(&self) -> &String{
        &self.discord_api
    }

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
