
use shiplift::{ rep::ContainerDetails };
//use sysinfo::{System, SystemExt};
use cgroups_fs::{ CgroupName, Cgroup };

#[derive(Debug)]
pub struct ContainerProcess<'a> {
    container: &'a ContainerDetails,
    cgroup_memory: Cgroup,
}

impl<'a> ContainerProcess<'a> {
    pub fn new(container: &'a ContainerDetails, cgroup_name: CgroupName) -> ContainerProcess<'a> {
        let cgroup_memory = cgroups_fs::Cgroup::new(&cgroup_name, "memory");
        ContainerProcess {
            container,
            cgroup_memory,
        }
    }
}