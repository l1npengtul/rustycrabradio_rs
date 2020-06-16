// The Main Server Thread per Server. This manages the playing, loading, etc of songs with a MPSC
// architecture from the main -> server thread.

use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::thread_interfacer::CommunicationType;


// intelliJ, can you shut the fuck up, im using 2018 already godammit
pub async fn music_play_thread(thread_name : String, mspc_recv : Receiver<CommunicationType>, mspc_tran : Sender<CommunicationType>){
    
}
