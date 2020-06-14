#[macro_use]
extern crate serenity;
extern crate typemap;

use std::{collections::{HashMap, HashSet}, sync::Arc};
use serenity::{
    async_trait,
    client::{
        bridge::voice::ClientVoiceManager,
        Context,
    },
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError,HelpBehaviour, help_commands, StandardFramework,
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        channel::{Channel, Message}, gateway::Ready, id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
    Result as SerenityResult,
};
use serenity::prelude::*;

use serenity_lavalink::{
    LavalinkClient,
    nodes::Node,
};
use std::sync::mpsc::channel;
use serenity::model::prelude::GuildId;
use tokio;


mod config_loader;

// IntelliJ-Rust is so slow i want to cri
// I cri everi tiem


struct SearchBoxMessage{

}
struct VoiceManager;
struct Lavalink;
impl TypeMapKey for VoiceManager {
    type Value = Arc<tokio::sync::Mutex<ClientVoiceManager>>;
}
impl TypeMapKey for Lavalink {
    type Value = Arc<tokio::sync::RwLock<LavalinkClient>>;
}


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
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Command Groups
#[group]
#[description="Commands relating to playing Music"]
#[only_in(guilds)]
//TODO: Add commands once finalized
#[commands(play,search,skip,fskip,ban_term,ban_link,unban_link,unban_term,list_banned)]
struct Music;

#[group]
#[description="Other Commands"]
#[commands(ping)]
struct Other;
/*

#[group]
#[description="Commands relating to Debug/Bot Info"]
#[commands()]
struct Debug;

#[group]
#[description="Commands relating to Bot Config/Shutdown,etc."]
#[owners_only]
struct Owner;
*/


// Hook Commands
// Run right before, after, when error occours, etc

// TODO: Make a function that returns the current date-time
// TODO: Make a logger

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Processing command {} by user {}", command_name, msg.author.id);
    true
}


#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Sucessfully Parsed Command"),
        Err(why) => eprintln!("Error Parsing Command: {:?}",why),
    }
}


#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    eprintln!("Unknown Command: {}",unknown_command_name);

}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    eprintln!("Error {} is not a command" , msg.content.as_str());
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) -> () {

    if let DispatchError::Ratelimited(seconds) = error {
        let _who_is_rem = msg.channel_id.say(&ctx.http, format!("Error: Rate Limited! {}s", seconds)).await;
        eprintln!("Rate limited! {}s", seconds);
    }
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let config = config_loader::get_config();
    let disc_token = config.get_discord_api();

    let http = Http::new_with_token(&disc_token);


    // lets get the bot owner id so we can execute the special commands
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let bot_id = match http.get_current_user().await {
        Ok(id)=>id.id,
        Err(why) => panic!("Error: Could not get the user connected info: {:?}", why),
    };
    let mut client = Client::new(config.get_discord_api()).event_handler(Handler).await.expect("Error Creating Client");
    {
        let mut data = client.data.write().await;
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));


        let mut lava_client = LavalinkClient::new();
        lava_client.bot_id = bot_id;
        lava_client.initialize().await.unwrap();
        data.insert::<Lavalink>(Arc::new(tokio::sync::RwLock::new(lava_client)));

    }
    let _rem_plays_twister = client.start().await.map_err(|why| eprintln!("Error: Client Ended: {:?}", why));
    Ok(())
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
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let i_love_emilia = args.message().to_string();

    let why_are_they_called_guilds_and_not_servers = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            chk_log(msg.channel_id.say(&ctx.http, "DMs are not supported by the `play` feature!").await);
            return Ok(());
        },
    };

    let mg_lc = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Error: No VoiceManager in TypeMap");
    let mut mgmt = mg_lc.lock().await;

    if let Some(handler) = mgmt.get_mut(why_are_they_called_guilds_and_not_servers) {
        if join_channel(ctx,msg).await{
            if play_music(ctx,msg, &i_love_emilia).await{
                return Ok(());
            }
            else{
                return Ok(());
            }
        }
        else {
            chk_log(msg.reply(&ctx.http,"Error: Could not join the VoiceChannel.").await);
            return Ok(());
        }

    }

    Ok(())



}

//TODO: implement these commands

#[command]
#[only_in(guilds)]
async fn search(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
// Initiates a Vote to Skip. Will get the total amount of people in a chat and only requires a set % of yes votes to skip
// TODO: Make vote % configurable
async fn skip(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
// Only admins can call this command, if they are not an admin they will not get a response
#[required_permissions("ADMINISTRATOR")]
//force skips current song
async fn fskip(ctx : &Context, msg: &Message, args: Args) -> CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn ban_term(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn ban_link(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn unban_term(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn unban_link(ctx : &Context, msg: &Message, args: Args)->CommandResult{
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn list_banned(ctx : &Context, msg: &Message)->CommandResult{
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) ->CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = chk_log(msg.reply(&ctx.http,"There was a problem getting the shard manager").await);

            return Ok(());
        },
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            eprintln!("Error: Could not get message, Shard retrieval most likely failed.");
            return Ok(());
        },
    };

    // stfu intellij its not mis-spelled
    let _emilia_hentai = msg.reply(&ctx.http,&format!("The shard latency is {:?}", runner.latency));
    Ok(())
}

fn chk_log(result: SerenityResult<Message>) {
    if let Err(why) = result {
        eprintln!("Error sending message: {:?}", why);
    }
}

// So both search and play go here, this will activate the lavalink player
async fn play_music(ctx : &Context,msg:&Message, query : &String)->bool{
    let data = ctx.data.read().await;
    let lava_lock = data.get::<Lavalink>().expect("Error: No Lavalink in TypeMap");
    let mut lava_client = lava_lock.write().await;

    let query_information = match lava_client.auto_search_tracks(&query).await{
        Ok(val) => val,
        Err(why)=> return false,
    };

    let guild_not_server_id = match msg.guild(&ctx.cache).await{
        Some(guild) => guild,
        None => {
            chk_log(msg.reply(&ctx.http, "Error: You need to be in a Guild!").await);
            return false;
        }
    };

    let guild_id = guild_not_server_id.id;
    // intelliJ can you fuck off its not wrong the compiler says its fine REEEEEEEEEEEE
    let manager_lock = ctx.data.read().await
        .get::<VoiceManager>().cloned().expect("Expected VoiceManager in TypeMap.");
    let mut manager = manager_lock.lock().await;

    if let Some(handler) = manager.get_mut(guild_id){
        let data = ctx.data.read().await;
        let lava_lock = data.get::<Lavalink>().expect("Expected a lavalink client in TypeMap");
        let mut lava_client = lava_lock.write().await;

        let query_information = match lava_client.auto_search_tracks(&query).await{
            Ok(track) => track,
            Err(why) => {
                chk_log(msg.channel_id.say(&ctx.http, format!("Error: Could not get track: {:?}",why)).await);
                eprintln!("Error: Could not get track: {:?}",why);
                return false;
            },
        };


        if query_information.tracks.is_empty() {
            chk_log(msg.channel_id.say(&ctx, "Could not find any video of the search query.").await);
            return false;
        }

        {
            // Note: This will panic if getting node returns an error. Too bad!
            let node = lava_client.nodes.get_mut(&guild_id).unwrap();
            node.play(query_information.tracks[0].clone()).queue();
        }
        let node = lava_client.nodes.get(&guild_id).unwrap();

        // start looping if there is nothing in there
        if !lava_client.loops.contains(&guild_id) {
            node.start_loop(Arc::clone(lava_lock), Arc::new(handler.clone())).await;
        }
        chk_log(msg.channel_id.say(&ctx.http, format!("Added to queue: {}", query_information.tracks[0].info.title)).await);
    }
    else {
        chk_log(msg.channel_id.say(&ctx.http, format!("Error: Not connected to VoiceChannel.")).await);
        return false;
    }


    true
}

async fn join_channel(ctx : &Context, msg : &Message) -> bool{
    let guild_not_server_id = match msg.guild(&ctx.cache).await{
        Some(guild) => guild,
        None => {
            chk_log(msg.reply(&ctx.http, "Error: You need to be in a Guild!").await);
            return false;
        }
    };

    let guild_id = guild_not_server_id.id;

    let vc_channel_id = guild_not_server_id.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);

    let connect_vc = match vc_channel_id{
        Some(vc)=>vc,
        None=>{
            chk_log(msg.reply(&ctx.http, "Error: You are not currently in a Voice Channel!").await);
            return false;
        }
    };

    let lck = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Error: Expected VoiceManager in typemap.");
    let mut manager = lck.lock().await;

    if manager.join(guild_id, connect_vc).is_some() {
        let data = ctx.data.read().await;
        let lava_client_lock = data.get::<Lavalink>().expect("Error: Expected a lavalink client in TypeMap");
        let mut lava_client = lava_client_lock.write().await;
        Node::new(&mut lava_client, guild_id, msg.channel_id);
    } else {
        chk_log(msg.channel_id.say(&ctx.http, "Error joining the channel").await);
        return false;
    }
    true


}