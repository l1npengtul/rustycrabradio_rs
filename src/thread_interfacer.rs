use serenity::prelude::Context;
use serenity::model::prelude::Message;
use crate::config_loader::Video;
use std::thread;

pub enum CommunicationType{
    GetMusic,
    AddMusic,
    ShutdownThread,
    Ok200,
}
pub enum SendRecv{
    Send,
    Recv,
}



pub struct ThreadCommunication {
    pub(crate) com_type : CommunicationType,
    pub(crate) com_t_type : SendRecv,
    pub(crate) com_message : String,
    pub(crate) com_video : Video,
}
impl ThreadCommunication{
    pub async fn new(t : CommunicationType, t_type : SendRecv, msg : String, vid : Video) -> Self{
        ThreadCommunication{
            com_type : t,
            com_t_type : t_type,
            com_message : msg,
            com_video : vid,
        }
    }
}

