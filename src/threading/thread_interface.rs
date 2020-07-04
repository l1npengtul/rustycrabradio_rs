

// Due to our architecture, we only need 4 commands:
// 1. Play/AddMusic
// 2. Skip
// 3. Seek
// 4. Die/Exit Thread
pub enum MessageType{
    Play,
    Skip,
    Seek,
    Exit,
}



pub struct ThreadComm{
    com_type : MessageType,
    video_optional : Option<Video>,
}