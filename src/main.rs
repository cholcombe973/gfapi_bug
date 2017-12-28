extern crate gfapi_sys;
use std::thread;

use gfapi_sys::gluster::*;

fn main() {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let cluster = match Gluster::connect("test", "localhost", 24007) {
                Ok(c) => c,
                Err(e) => {
                    println!("{:?}", e);
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
