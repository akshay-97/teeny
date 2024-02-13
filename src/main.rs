extern crate stats_alloc;
use std::net::{TcpStream, TcpListener};
use std::io::{prelude::*, BufReader};
use std::{thread, time::Duration};

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

mod pool;
pub mod gru;

fn handle_connect(mut stream: TcpStream,) -> &'static str{
    let buf_reader = BufReader::new(&mut stream);
    thread::sleep(Duration::from_millis(4000));
    let _: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    return "asdc"
}

pub fn main(){
    let region = Region::new(&GLOBAL);
    //let (tx, rx) = mpsc::channel();
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    let mut threadP = pool::ThreadPool::new(5);
    
    for stream in listener.incoming(){
        println!("Stats at 1: {:#?}", region.change());
        match stream {
            Ok(stream) => 
                {
                    let _ = threadP.execute(|| { handle_connect(stream);});
                    ()
                },
            Err(e) => {panic!(" stream error")}
        }
        println!("Stats at 2: {:#?}", region.change());
    }

}

// pub struct str_split{
//     delim : &'a str,
//     rem : &'a str
// }

// impl Iterator for str_split {
//     pub fn next(&mut self) -> Option<Item> {
//         type Item = &str;

//     }
// }
// pub fn main(){
//     gru::element::make()
// }