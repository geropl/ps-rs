extern crate shiplift;
extern crate sysinfo;
extern crate tokio;
extern crate cgroups_fs;
extern crate signal_hook;

use std::{ error::Error, result::Result };
use std::{ thread, time::Duration };
use std::vec::Vec;
use shiplift::{ ContainerListOptions, rep::Container, rep::ContainerDetails };
use shiplift::Docker;
//use sysinfo::{System, SystemExt};
use tokio::prelude::{ Future, future::join_all };

mod container_process;
use container_process::ContainerProcess;

// [0].HostConfig.CgroupParent

// docker ps -> docker id -> container

fn main() {
    //let id = env::args().nth(1).expect("Usage: cargo run -- <container>");

    install_signal_handler().expect("Error installing signal handler");

    let docker_client = Docker::new();
    loop {
        let filter_options = ContainerListOptions::builder().all().build();
        let res = fetch_container_details(&docker_client, &filter_options);
        if !res.is_err() {
            handle_containers(&res.unwrap());
        } else {
            println!("Error checking containers: {:?}", res.err());
        }

        thread::sleep(Duration::from_millis(100));
    }

    // // Memory information:
    // println!("total memory: {} kB", sys.get_total_memory());
    // println!("used memory : {} kB", sys.get_used_memory());
    // println!("total swap  : {} kB", sys.get_total_swap());
    // println!("used swap   : {} kB", sys.get_used_swap());
}

fn fetch_container_details(docker_client: &Docker, filter_options: &ContainerListOptions) -> Result<Vec<ContainerDetails>, shiplift::Error> {
    let f = docker_client.containers().list(filter_options)
        .map(|cs: Vec<Container>| {
            let future_container_details = cs.into_iter()
                .map(|c|
                    docker_client.containers()
                        .get(&c.id)
                        .inspect()
                );
            join_all(future_container_details).wait()
        });
    match f.wait() {
        Ok(details) => details,
        Err(e) => Err(e)
    }
}

fn handle_containers(containers: &Vec<ContainerDetails>) {
    for container in containers {
        map_container_to_cgroup(&container);
    }
}

fn map_container_to_cgroup(container: &ContainerDetails) -> Option<ContainerProcess> {
    match &container.host_config.cgroup_parent {
        None => {
            println!("Container has no cgroup_parent: {:#?}", container.name);
            None
        },
        Some(cgroup_parent) => {
            println!("Found container {:?}/{:?} with cgroup_parent {:?}", container.name, container.id, cgroup_parent);
            let cgroup_name = cgroups_fs::CgroupName::new(cgroup_parent);
            Some(ContainerProcess::new(container, cgroup_name))
        }
    }
    //     .map(|cgroup_parent| {
    //         let mut sys = System::new();
    //         sys.refresh_processes();
    //         for process in sys.get_process_list() {
    //             let (_pid, p) = process;
    //             println!("{}", p.name);
    //             println!("{:?}", process);
    //         }

    //         println!("{:#?}", &cgroup_parent);
    //         let name = cgroups_fs::CgroupName::new(cgroup_parent);
    //         let group_memory = cgroups_fs::Cgroup::new(&name, "memory");
    //         let mem_limit_in_bytes: u64 = group_memory
    //             .get_value("memory.limit_in_bytes")
    //             .expect("Cannot read memory.limit_in_bytes");
    //         println!("memory.limit_in_bytes: {}", mem_limit_in_bytes);
    //     })
    //     .map_err(|e| eprintln!("Error: {}", e));
}

fn install_signal_handler() -> Result<(), Box<Error>> {
    use std::process;
    use signal_hook::{iterator::Signals, SIGINT};

    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}, exiting.", sig);
            process::exit(1);
        }
    });

    Ok(())
}
