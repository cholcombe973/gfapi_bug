extern crate gfapi_sys;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::thread;

use gfapi_sys::gluster::*;
use simplelog::{Config, SimpleLogger};

fn main() {
    let _ = SimpleLogger::init(log::LogLevelFilter::Trace, Config::default()).unwrap();
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let cluster = match Gluster::connect("test", "localhost", 24007) {
                Ok(c) => c,
                Err(e) => {
                    error!("{:?}", e);
                    return;
                }
            };
            let result = cluster.get_volume_id();
            println!("cluster vol id: {:?}", result);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join();
    }
}
