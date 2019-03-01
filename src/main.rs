extern crate shiplift;
// extern crate sysinfo;

use shiplift::{ Docker };
// use sysinfo::{ SystemExt, System };

fn main() {
    let docker = Docker::new();
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
