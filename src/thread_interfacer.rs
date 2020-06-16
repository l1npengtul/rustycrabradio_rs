
pub enum CommunicationType{
    GetMusic,
    AddMusic,
    SetMusic,
    ShutdownThread,
}
pub enum SendRecv{
    Send,
    Recv,
}

pub struct ThreadCommunication {
    com_type : CommunicationType,
    com_t_type : SendRecv,
    com_message : String,
}
impl ThreadCommunication{
    pub async fn new(t : CommunicationType, t_type : SendRecv, msg : String) -> Self{
        ThreadCommunication{
            com_type : t,
            com_t_type : t_type,
            com_message : msg,
        }
    }
}