// The Main Server Thread per Server. This manages the playing, loading, etc of songs with a MPSC
// architecture from the main -> server thread.

use std::{thread,time};
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use crate::thread_interfacer::*;
use crate::config_loader::*;
use serenity::prelude::{Mutex, Context, Mentionable, RwLock};
use serenity::model::prelude::Message;
use serenity::client::Cache;
use serenity::framework::standard::CommandResult;
use crate::{VoiceManager, error};
use crate::chk_log;
use serenity::voice;
use serenity::voice::{LockedAudio, Audio, AudioSource, Handler, Manager};
use std::sync::Arc;
use std::ops::Deref;
use std::time::{Duration, Instant};
use crate::error::HandlerError::HandlerGetError;
use crate::error::VideoError;
use crate::join_channel;

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
    let mut music_queue : Vec<Video> = vec![];
    let mut music_set : bool;
    let mut music_drought : i64 = 0;
    let mut music_timer : Duration = Duration::from_secs(0);
    let mut instant : Instant = Instant::now();
    let mut handler = match get_handler(ctx,msg).await {
        Ok(handle) => handle,
        Err(why) => {
            eprintln!("Error getting handler: {:?}",why);
            return;
        }
    };
    if join_sucess{

        'main_loop : loop {
            if let Some(v) = read_recv(recv).await{
                music_queue.push(v);
            }
            if music_queue.len() > 0 {
                if let Some(emimusicemimusic) = music_queue.get(0){
                    if let Ok(src) = get_source_ytdl(emimusicemimusic.link.as_ref()).await {
                        println!("Currently Playing: {}", emimusicemimusic.title);
                        chk_log(msg.channel_id.say(&ctx.http, format!("Currently Playing: {}", emimusicemimusic.title)).await);

                        let length = match emimusicemimusic.length.as_str(){
                            Some(s) => s,
                            None => "Ok Boomer",
                        };
                        let length_u64 = match emimusicemimusic.length.as_u64(){
                            Some(s) => s,
                            None => 0,
                        };

                        handler.play_only(src);
                        println!("Secs: {}", length);
                        music_timer = Duration::from_secs(length_u64);
                        instant = Instant::now();
                    }
                }
            }
            else {
                break;
            }
            'timer_loop : loop {
                let mut recv_video : Video;
                if let Some(v) = read_recv(recv).await{
                    recv_video = v;
                    music_queue.push(recv_video);
                }

                if instant.elapsed() > music_timer {
                    music_queue.pop();
                    break 'timer_loop;
                }
            }
        }
    }
    else {
        eprintln!("Error joining voicechannel");
    }
}

async fn read_recv(recv : &Receiver<ThreadCommunication>) -> Option<Video> {
    let com = match recv.recv() {
        Ok(i_love_emilia) => i_love_emilia,
        Err(why) => { return None }
    };
    match com.com_type {
        CommunicationType::AddMusic => {
            return Some(com.com_video)
        }
        _ => { return None }
    }
}

// A Strict containing a mutable struct because im too lazy to do &'a bs
pub struct HandlerContainer {
    pub(crate) handler : Handler,
}

async fn get_handler(ctx: &Context, msg : &Message) -> Result<Box<&mut Handler>,error::HandlerError>{
    let guild_id = match msg.guild_id {
        Some(emiguildid) => emiguildid,
        None => {return Err(HandlerGetError { why: "Guild ID returned None ".to_string() });}
    };

    let mut a = match ctx.data.read().await.get::<VoiceManager>().cloned(){
        Some(a) => a,
        None => {return Err(HandlerGetError {why : String::from("Reading handler returned null")})}
    };
    let mut manager = a.lock().await;


    if let Some(handler) = manager.get_mut(guild_id){
        return Ok(handler);
    }

    Err(HandlerGetError {why : "Failed to lock handler.".to_string()})
}

async fn get_source_ytdl(link : &str) -> Result<Box<dyn AudioSource>, error::VideoError>{
    let source = match voice::ytdl(link).await{
        Ok(src) => src,
        Err(why) => {
            return Err(error::VideoError::VideoSourceError {
                link : String::from(link),
                reason : why.to_string(),
            })
        }
    };
    Ok(source)
}



fn leave_channel(ctx : &Context, msg : &Message) -> CommandResult{
    Ok(())
}