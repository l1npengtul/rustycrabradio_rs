#[macro_use]
extern crate serenity;
extern crate typemap;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        channel::{Channel, Message}, gateway::Ready, id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
};
use serenity::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Datelike, Timelike};

use serenity_lavalink::{
    LavalinkClient,
    nodes::Node,
};
use tokio::sync::RwLock;
use std::sync::mpsc::channel;

mod config_loader;

// IntelliJ is so slow i want to cri
// I cri everi tiem


struct Lavalink;

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

    let http = Http::new_with_token(&token);


    // lets get the bot owner id so we can execute the special commands
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
            //TODO: Allow Prefix Configuration
            .configure(|c| c
                .allow_whitespace(true)
                .on_mention(true)
                .prefix("m!")
                .delimiters(",")
                .owners(owners))
            .before(before)
            .after(after)
            .unrecognised_command(unknown_command)
            .normal_message(normal_message)
            .on_dispatch_error(dispatch_error)
        .help(&HELP_CMD)
        .group(&MUSIC_GROUP)
        .group(&DEBUG_GROUP)
        .group(&OWNER_GROUP)
        .group(&OTHER_GROUP);

    let mut client = Client::new(config.get_discord_api())
        .event_handler(Handler)
        .framework(framework)
        .await
        .except("Error creating client!");
    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));

        let mut lava_client = LavalinkClient::new();
        lava_client.bot_id = bot_id;
        lava_client.initialize().await?;
        data.insert::<Lavalink>(Arc::new(tokio::sync::RwLock::new(lava_client)));

    }
    if let Err(why) = client.start().await{
        println!("[{}/{:02}/{:02} {:?}- {:02}:{:02}:{:02}]: ERROR: {:?}",
             year,
             now.month(),
             now.day(),
             // just in case we have time travellers
             if is_common_era{"CE"} else BCE{"BCE"},
             hour,
             now.minute(),
             now.day(),
             now.second(),
             why
             );
    }




}
/*
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
*/

// Command definitions start here
// Current Commands: play, search, skip, ping, queue

/*
#[command]
async fn example(ctx : &Context, msg: &Message, args: Args) -> CommandResult{

}
basic command template, ur welcome future lewis
 */


#[command]
//servers (guilds) only
#[only_in(guilds)]
#[min_args(1)]
async fn play(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult{
    let i_love_emilia = args.message().to_string();

    let why_are_they_called_guilds_and_not_servers = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            chk_log(msg.channel_id.say(&ctx.http, "DMs are not supported by the `play` feature!"));
            return Ok(());
        },
    };

    //Automatically Join the VoiceCHannel the user is in
    let user_to_isekai = &msg.author;
    let user_voice_channel = user_to_isekai.


    let mg_lc = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Error: No VoiceManager in TypeMap");
    let mut mgmt = mg_lc.lock().await;

    if let Some(handler) = mgmt.get_mut(why_are_they_called_guilds_and_not_servers) {
        let data = ctx.data.read().await;
        let lava_lock = data.get::<Lavalink>().expect("Error: No Lavalink in TypeMap");
        let mut lava_client = lava_lock.write().await;

        let query_information = lava_client.auto_search_tracks(&query).await?;

        if query_information.tracks.is_empty() {
            check_msg(msg.channel_id.say(&ctx, "Could not find any video of the search query.").await);
            return Ok(());
        }

        {
            let node = lava_client.nodes.get_mut(&why_are_they_called_guilds_and_not_servers).unwrap();

            node.play(query_information.tracks[0].clone())
                //.start_time(Duration::from_secs(61))
                //.replace(true)
                .queue();
        }
        let node = lava_client.nodes.get(&why_are_they_called_guilds_and_not_servers).unwrap();

        if !lava_client.loops.contains(&why_are_they_called_guilds_and_not_servers) {
            node.start_loop(Arc::clone(lava_lock), Arc::new(handler.clone())).await;
        }

        check_msg(msg.channel_id.say(&ctx.http, format!("Added to queue: {}", query_information.tracks[0].info.title)).await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Use `~join` first, to connect the bot to your current voice channel.").await);
    }
    Ok(())



}


#[command]
#[only_in(guilds)]
async fn search(ctx : &Context, msg: &Message, args: Args){

}

#[command]
async fn skip(ctx : &Context, msg: &Message, args: Args){

}

#[command]
async fn ping(ctx : &Context, msg: &Message){

}

fn chk_log(result: SerenityResult<Message>) {
    if let Err(why) = result {
        eprintln!("Error sending message: {:?}", why);
    }
}
