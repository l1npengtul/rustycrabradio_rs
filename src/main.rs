use std::{env, sync::Arc};

use serenity::client::bridge::voice::ClientVoiceManager;

use serenity::{client::Context, prelude::Mutex};

use serenity::{
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready, misc::Mentionable},
    Result as SerenityResult,
    voice,
};

use serenity::prelude::*;
use std::collections::HashSet;

struct VoiceManager;

mod config_loader;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(help,play,search,now_playing,performance,skip,fskip,queue)]
struct General;

fn main() {
    let current_config = config_loader::get_config();

    let mut client = Client::new(&current_config.get_discord_api(), Handler).expect("Couldn't create client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    //TODO: ADD CUSTOMIZABLE BOT PREFIX, DELIMITERS
    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .on_mention(Some(bot_id))
            .with_whitespace(true)
            .delimiter(",")
            .owners(owners)
            .prefix("rb!"))


        .group(&GENERAL_GROUP));

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

#[command]
fn help(ctx: &mut Context, msg: &Message)->CommandResult{
    if let Err(why) = msg.channel_id.say(&ctx.http, "
    ```
    play [url] : Plays the song at the YouTube URL : aliases \"p\"
    search [query] : Searches for the song using the query on youtube : aliases \"query\"
    now_playing : Displays Currently Playing Song : aliases \"np\"
    skip : Votes to skip current song : aliases \"sk\"
    fskip : (ADMINISTRATORS ONLY) Force skips current song : aliases \"fsk\"
    queue : Displays Current Song Queue : aliases \"q\"\
    ```
    ")
    {
        println!("Error sending help message: {:?}", why);
    }
    Ok(())
}

#[command]

