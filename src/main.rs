extern crate tokio;
extern crate shiplift;
// extern crate sysinfo;

use std::env;
use tokio::prelude::Future;
use shiplift::{ Docker };
// use sysinfo::{ SystemExt, System };

fn main() {
    let id = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <container>");

    let docker = Docker::new();
    let f = docker.containers()
        .get(&id)
        .top(Option::None)
        .map(|top| println!("{:#?}", top))
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(f);

    // let mut sys = System::new();
    // sys.refresh_processes();

    // // Components temperature:
    // for process in sys.get_process_list() {
    //     println!("{:?}", process);
    // }

    // // Memory information:
    // println!("total memory: {} kB", sys.get_total_memory());
    // println!("used memory : {} kB", sys.get_used_memory());
    // println!("total swap  : {} kB", sys.get_total_swap());
    // println!("used swap   : {} kB", sys.get_used_swap());
}
