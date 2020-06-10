#[macro_use]
extern crate serenity;
extern crate typemap;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::{Args, DispatchError, StandardFramework, HelpBehaviour, CommandOptions, help_commands, HelpOptions, CommandGroup, CommandResult};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::Permissions;
use serenity::prelude::Mutex;
use serenity::prelude::*;
use typemap::Key;
use serenity::prelude::*;
use chrono::{Datelike, Timelike, Utc};
use serenity::model::prelude::UserId;

mod config_loader;

// IntelliJ is so slow i want to cri
// I cri everi tiem


struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer{
    //TODO: dirty hack, plox fix
    type Value = Arc<serenity::prelude::Mutex<ShardManager>>;
}

struct CommandCounter;
impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Command Groups
#[group]
#[description="Commands relating to playing Music"]
#[only_in(guilds)]
//TODO: Add commands once finalized
#[commands()]
struct Music;

#[group]
#[description="Other Commands"]
#[commands()]
struct Other;

#[group]
#[description="Commands relating to Debug/Bot Info"]
#[commands{}]
struct Debug;

#[group]
#[description="Commands relating to Bot Config/Shutdown,etc."]
#[owners_only]
struct Owner;

// Help Command
#[help]
//TODO: Allow Command Tip configuration
#[individual_command_tip="I Love Emilia!\n If you want more information about a command, just pass it as an argument!"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn help_cmd(
    ctx: &Context,
    msg: &Message,
    args: Args,
    hlp_opt: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult{
    help_cmd::with_embeds(ctx,msg,args,hlp_opt,groups,owners).await
}

// Hook Commands
// Run right before, after, when error occours, etc

//TODO: Make a function that returns the current date-time
//TODO: Make a logger
#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    let now = Utc::now();
    let hour = now.hour();
    let (is_common_era, year) = now.year_ce();
    println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: User {} issued command {}",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             command_name,
             msg.author.name
    );
    true
}


#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    let now = Utc::now();
    let hour = now.hour();
    let (is_common_era, year) = now.year_ce();
    match command_result {
        Ok(()) => println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: Successfully processed command {}",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             command_name
    ),
        Err(why) => println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: Failed to process command {} due to {:?}",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             command_name,
             why
    ),
    }
}


#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    let now = Utc::now();
    let hour = now.hour();
    let (is_common_era, year) = now.year_ce();
    println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: Unknown command '{}'",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             unknown_command_name
    );
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    let now = Utc::now();
    let hour = now.hour();
    let (is_common_era, year) = now.year_ce();
    println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: '{}' is not a command",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             msg.content
    );
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) -> () {
    let now = Utc::now();
    let hour = now.hour();
    let (is_common_era, year) = now.year_ce();
    if let DispatchError::Ratelimited(seconds) = error {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: Rate Limited! Trying again in {} seconds!",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             seconds)).await;
    };
}

#[tokio::main]
async fn main() {
    let config = config_loader::get_config();
    let disc_token = config.get_discord_api();

    let mut client = Client::new(disc_token.to_owned().as_str()).expect("Err creating client");
    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager))
    }

    client.with_framework(
        StandardFramework::new()
            //TODO: Allow Prefix Configuration
            .configure(|c| c
                .allow_whitespace(true)
                .on_mention(true)
                .prefix("m!")
                .delimiters(","))
            .before(|ctx,msg,command_name|{

            })

            )
    )

}

#[command]
//TODO: IMPROVE HELP
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



