// The Main Server Thread per Server. This manages the playing, loading, etc of songs with a MPSC
// architecture from the main -> server thread.

use std::{thread,time}
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use crate::thread_interfacer::*;
use crate::config_loader::*;
use serenity::prelude::{Mutex,Context};
use serenity::model::prelude::Message;
use serenity::client::Cache;
use serenity::framework::standard::CommandResult;
use crate::VoiceManager;
use crate::chk_log;
use serenity::voice;
use serenity::voice::{LockedAudio, Audio};
use std::sync::Arc;
use std::ops::Deref;
use std::time::Duration;

/*
pub async fn music_play_thread(ctx: &Context, msg : &Message, thread_name : String, data_recv : Receiver<ThreadCommunication>, data_send : Sender<ThreadCommunication>){
    let mut thread_st_aaaaaaaaaaaaaay_alive: bool = true;
    let mut queue = Vec::new();
    let mut audio_src : Audio;
    while thread_st_aaaaaaaaaaaaaay_alive {
        let msg_recv = match data_recv.recv() {
            Ok(com) => com,
            Err(why) => {
                // do nothing if there are no messages
            }
        };

        match msg_recv.com_type {
            CommunicationType::GetMusic=> {
                if let Some(v) = queue.get(0){
                    data_send.send(ThreadCommunication{
                        com_type: CommunicationType::Ok200,
                        com_t_type: SendRecv::Send,
                        com_message: "".to_string(),
                        com_video:
                    })
                }
            }
            CommunicationType::AddMusic => {
                queue.push(msg_recv.com_video);
                data_send.send(ThreadCommunication{
                    com_type: CommunicationType::Ok200,
                    com_t_type: SendRecv::Send,
                    com_message: "Sucessfully Added Music!".to_string(),
                    com_video: Video{
                        title: "".to_string(),
                        link: "".to_string(),
                        author: "".to_string(),
                        id: "".to_string(),
                        thumbnail: "".to_string(),
                        length: Default::default()
                    }
                });

            }
            CommunicationType::ShutdownThread => {
                thread_st_aaaaaaaaaaaaaay_alive = false;
                // add function to leave
                data_send.send(ThreadCommunication{
                    com_type: CommunicationType::Ok200,
                    com_t_type: SendRecv::Send,
                    com_message: "Shutting Down Thread...".to_string(),
                    com_video: Video {
                        title: "".to_string(),
                        link: "".to_string(),
                        author: "".to_string(),
                        id: "".to_string(),
                        thumbnail: "".to_string(),
                        length: Default::default()
                    }
                });
                continue;
            }
            _ => {}
        }
        if join_channel(ctx,msg).await == false{
            eprintln!("Could not join voice channel");
        }
        else {
            println!("Sucessfully joined voicechannel");

            if audio_src.playing == true{
                continue;
            }
            else{
                let guild_id = match ctx.read().guild_channel(msg.channel_id) {
                    Some(channel) => channel.read().guild_id,
                    None => {
                        chk_log(msg.channel_id.say(&ctx.http,"Error finding channel info"));
                    },
                };

                let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
                let mut manager = manager_lock.lock();

                if let Some(handler) = manager.get_mut(guild_id) {
                    let source = match voice::ytdl(queue.get(0).unwrap().link.as_str()) {
                        Ok(source) => source,
                        Err(why) => {
                            println!("Err starting source: {:?}", why);

                            chk_log(msg.channel_id.say(&ctx.http,"Error sourcing ffmpeg"));
                        },
                        _ => {
                            eprintln!("Failed to match");
                        }
                    };
                    let a_lck = handler.play_only(source).clone();
                    audio_src = a_lck.lock();

                    chk_log(msg.channel_id.say(&ctx.http,"Playing song"));
                } else {
                    chk_log(msg.channel_id.say(&ctx.http,"Not in a voice channel to play in"));
                }
            }
        }
    }
}*/


pub(crate) fn start_music(ctx : &Context, msg : &Message, recv :  &Receiver<ThreadCommunication>){

}

async fn play_music_thread(ctx : &Context, msg : &Message, recv : &Receiver<ThreadCommunication>){
    thread::sleep(time::Duration::from_millis(100));
    let join_sucess = join_channel(ctx,msg).await;
    let mut music_queue : Vec<Video>;
    let mut music_set : bool;
    let mut music_drought : i64 = 0;
    let mut music_timer : Duration;
    if join_sucess{

        'main_loop : loop {
            let cur_video = match read_recv(recv).await{
                Some(v) => v,
                None => (),
            };
            music_queue.push(cur_video);

            if music_queue.len() > 0 {

            }

            //Play_Only if items in queue
                // no items in queue = quit
            //set timer
            'timer_loop : loop {
                // read_recv()

                // see if timer expired
                    // true => break 'timer_loop
                    // else => continue
            }
        }
    }
    else {
        eprintln!("Error joining voicechannel");
    }
}

async fn read_recv(recv : &Receiver<ThreadCommunication>) -> Option<Video>{
    let com = match recv.recv() {
        Ok(i_love_emilia) => i_love_emilia,
        Err(why) => {return None}
    };
    match com.com_type {
        CommunicationType::AddMusic => {
            return Some(com.com_video)
        }
        _ => {return None}
    }
}

async fn get_source(ctx : &Context, msg : &Message) -> Result<>{

}

async fn join_channel(ctx : &Context, msg : &Message) -> bool{
    let guild = match msg.guild_id() {
        Some(guild) => guild,
        None => {
            chk_log(msg.channel_id.say(&ctx.http,"Groups and DMs not supported"));

            return false;
        }
    };

    let channel_id = guild
        .read()
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let guild_id = guild.read().id;


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            chk_log(msg.reply(&ctx.http,"Not in a voice channel"));

            return false;
        }
    };


    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock();

    if manager.join(guild_id, connect_to).is_some() {
        chk_log(msg.channel_id.say(&ctx.http,&format!("Joined {}", connect_to.mention())));
        return true;
    } else {
        chk_log(msg.channel_id.say(&ctx.http,"Error joining the channel"));
        return false;
    }
}

fn leave_channel(ctx : &Context, msg : &Message) -> CommandResult{
    Ok(())
}