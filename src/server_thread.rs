// The Main Server Thread per Server. This manages the playing, loading, etc of songs with a MPSC
// architecture from the main -> server thread.

use std::thread;
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};
use crate::thread_interfacer::*;
use crate::config_loader::*;
use serenity::prelude::Context;
use serenity::model::prelude::Message;


pub async fn music_play_thread(ctx: Context, msg : Message, thread_name : String, data_recv : Receiver<ThreadCommunication>, data_send : Sender<ThreadCommunication>){
    let mut thread_st_aaaaaaaaaaaaaay_alive: bool = true;
    let mut queue : Vec<Video> = Vec::new();
    while thread_st_aaaaaaaaaaaaaay_alive {
        let msg_recv = match data_recv.recv() {
            Ok(com) => com,
            Err(why) => {
                // do nothing if there are no messages
            }
        };
        match msg_recv.com_type {
            GetMusic=> {
                if let Some(v) = queue.get(0){
                    let a = com.clone();
                    data_send.send(ThreadCommunication{
                        com_type: CommunicationType::Ok200,
                        com_t_type: SendRecv::Send,
                        com_message: "".to_string(),
                        com_video: *v
                    })
                }
            },
            SetMusic => {

            }
        }

        // TODO: implement a check to see if message is for thread or not

        queue.push(msg_recv);




    }
}
