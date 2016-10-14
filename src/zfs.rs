use std::process::{Command, Stdio};
use std::fs::read_dir;

#[derive(Debug)]
pub struct Volume {
    pub name: String,
    pub path: String,
}

impl Volume {
    pub fn new<S: Into<String>>(name: S, path: S) -> Volume {
        Volume {
            name: name.into(),
            path: path.into(),
        }
    }
}

pub fn get_volumes() -> Vec<Volume> {
    let mount = Command::new("mount")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute mount");

    let output = mount.wait_with_output()
        .expect("failed to wait on mount");

    let mut volumes = Vec::new();

    if let Ok(mounted_volumes) = String::from_utf8(output.stdout) {
        for volume in mounted_volumes.lines() {
            if volume.contains("zfs") {
                let volume: Vec<String> = volume.split_whitespace()
                    .map(|s| s.into())
                    .collect();
                assert!(volume.len() == 6);
                volumes.push(Volume::new(volume[0].clone(), volume[2].clone()));
            }
        }
    }

    volumes
}

#[derive(Debug)]
pub struct Snapshot {
    pub path: String,
    pub name: String,
}

impl Snapshot {
    pub fn new<S: Into<String>>(path: S, name: S) -> Snapshot {
        Snapshot {
            path: path.into(),
            name: name.into(),
        }
    }
}

pub fn get_snapshots<S: Into<String>>(path: S) -> Vec<Snapshot> {
    let mut snapshots = Vec::new();

    if let Ok(entries) = read_dir(format!("{}/{}", path.into(), ".zfs/snapshot/")) {
        for entry in entries {
            if let Ok(entry) = entry {
                snapshots.push(Snapshot::new(entry.path()
                                                 .to_str()
                                                 .expect("snapshot path not readable"),
                                             entry.file_name()
                                                 .to_str()
                                                 .expect("snapshot name not readable")));
            }
        }
    }

    snapshots
}
