use crate::linux::cap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    process: Process,
    root: Root,
    hostname: String,
    domainname: String,
    mounts: Vec<Mount>,
    annotations: HashMap<String, String>,
    linux: Linux,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    #[serde(rename = "uidMappings")]
    uid_mappings: LinuxIDMapping,
    #[serde(rename = "gidMappings")]
    gid_mappings: LinuxIDMapping,

    sysctl: HashMap<String, String>,

    namespaces: Vec<LinuxNamespace>,
    devices: Vec<LinuxDevice>,
    //TODO: add more
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    uid: u32,
    gid: u32,
}

impl Default for User {
    fn default() -> Self {
        User { uid: 0, gid: 0 }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    #[serde(default)]
    terminal: bool,
    #[serde(default)]
    user: User,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
    #[serde(default)]
    cwd: String,
    #[serde(default)]
    capabilities: cap::LinuxCapabilities,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    path: PathBuf,
    readonly: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIDMapping {
    #[serde(rename = "containerID")]
    container_id: u32,
    #[serde(rename = "hostID")]
    host_id: u32,
    size: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    destination: PathBuf,

    #[serde(rename = "type")]
    mount_type: String,

    source: PathBuf,

    #[serde(rename = "uidMappings")]
    uid_mappings: LinuxIDMapping,
    #[serde(rename = "gidMappings")]
    gid_mappings: LinuxIDMapping,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxResources {
    devices: Vec<LinuxDeviceCgroup>,
    network: LinuxNetwork,
    //TODO: add more
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDeviceCgroup {
    allow: bool,

    #[serde(rename = "type")]
    device_type: String,

    major: u64,
    minor: u64,
    access: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMode {
    mode: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDevice {
    path: PathBuf,

    #[serde(rename = "type")]
    device_type: String,

    major: u64,
    minor: u64,

    #[serde(rename = "fileMode")]
    file_mode: FileMode,

    uid: u32,
    gid: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxInterfacePriority {
    name: String,
    peioeiry: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNetwork {
    #[serde(rename = "classID")]
    class_id: u32,

    priorities: Vec<LinuxInterfacePriority>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxNamespaceType {
    #[serde(rename = "pid")]
    PIDNamespace,
    #[serde(rename = "network")]
    NetworkNamespace,
    #[serde(rename = "mount")]
    MountNamespace,
    #[serde(rename = "ipc")]
    IPCNamespace,
    #[serde(rename = "uts")]
    UTSNamespace,
    #[serde(rename = "user")]
    UserNamespace,
    #[serde(rename = "cgroup")]
    CgroupNamespace,
    #[serde(rename = "time")]
    TimeNamespace,
    #[serde(rename = "bpf")]
    BPFNamespace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    namespace_type: LinuxNamespaceType,

    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use serde_json::json;

    use super::*;

    #[test]
    fn json() {
        let n = json!({
            "type": "pid",
            "path": "/run/runc"
        });

        let j: LinuxNamespace = serde_json::from_str(&n.to_string()).unwrap();
        println!("{:?}", j)
    }
}
