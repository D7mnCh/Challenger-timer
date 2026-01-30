use std::thread;
use std::time::Duration;
use std::sync::mpsc::*;

pub fn spawn_thread_counter(tx: Sender<i32>){ 
    let timer = thread::spawn(move|| {
        let mut seconds = Box::new(0);
        loop {
            thread::sleep(Duration::from_secs(1));
            *seconds += 1;
            println!("this was printed from thread : {}",seconds);
            tx.send(*seconds).expect("couldn't not send the value to the main thread");
        }
    });
}
pub fn receive_secs(rx: Receiver<i32>)-> i32{
    let seconds = rx.recv().expect("couldn't not received the value from spawned thread");
    println!("this was printed from main: {}",seconds);
    return seconds;
    //thread::sleep(Duration::from_secs(1));

}
