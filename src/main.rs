extern crate tokio;
extern crate shiplift;
extern crate sysinfo;
//extern crate cgroup_fs;

use std::env;
use tokio::prelude::Future;
use shiplift::{ Docker };
use shiplift::rep::{ ContainerDetails };
use sysinfo::{ SystemExt, System };

use cgroups_fs;
// [0].HostConfig.CgroupParent

// docker ps -> docker id -> container
fn main() {
    let id = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <container>");

    let docker = Docker::new();
    let f = docker.containers()
        .get(&id)
        .inspect()
        .map(|details: ContainerDetails| {
            println!("{:#?}", details);
            details.host_config.cgroup_parent.expect("cgroup_parent not set")
        })
        .map(|cgroup_parent| {

            let mut sys = System::new();
            sys.refresh_processes();
            for process in sys.get_process_list() {
                let (_pid, p) = process;
                println!("{}", p.name);
                println!("{:?}", process);
            }


            println!("{:#?}", &cgroup_parent);
            let name = cgroups_fs::CgroupName::new(cgroup_parent);
            let group_memory = cgroups_fs::Cgroup::new(&name, "memory");
            let mem_limit_in_bytes: u64 = group_memory.get_value("memory.limit_in_bytes").expect("Cannot read memory.limit_in_bytes");
            println!("memory.limit_in_bytes: {}", mem_limit_in_bytes);
        })
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
