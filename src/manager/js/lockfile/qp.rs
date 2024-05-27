use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::manager::js::obtain::Package;
use rkyv::{Archive, Deserialize, Serialize};

const LOCKFILE_NAME: &str = "qp.lock";

fn write_to_lockfile(package_bytes: Vec<u8>) {
    let mut lockfile = OpenOptions::new()
        .create(true)
        .write(true)
        .open(LOCKFILE_NAME)
        .unwrap();
    lockfile.write_all(package_bytes.as_slice()).unwrap();
    lockfile.sync_all().unwrap();
}

#[archive(check_bytes)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
struct Lockfile {
    pub packages: Vec<Package>,
}

pub fn add_package_to_lockfile(package: Package) {
    if !Path::new(LOCKFILE_NAME).exists() {
        File::create(LOCKFILE_NAME).unwrap();
    }

    let mut lockfile = File::open(LOCKFILE_NAME).unwrap();

    let metadata = fs::metadata(LOCKFILE_NAME).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    lockfile.read(&mut buffer).unwrap();

    let mut bytes: Lockfile;

    if buffer.is_empty() {
        bytes = Lockfile {
            packages: vec![package],
        };

        let bytes = rkyv::to_bytes::<_, 256>(&bytes).unwrap().to_vec();

        write_to_lockfile(bytes);
    } else {
        bytes = rkyv::from_bytes(&buffer).unwrap();

        if bytes.packages.iter().any(|p| p.name == package.name) {
            return;
        }

        bytes.packages.push(package);

        let bytes = rkyv::to_bytes::<_, 256>(&bytes).unwrap().to_vec();

        write_to_lockfile(bytes);
    }
}

pub fn get_package_from_lockfile(name: &str) -> Option<Package> {
    if !Path::new(LOCKFILE_NAME).exists() {
        File::create(LOCKFILE_NAME).unwrap();
    }

    let mut lockfile = File::open(LOCKFILE_NAME).unwrap();

    let metadata = fs::metadata(LOCKFILE_NAME).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    lockfile.read(&mut buffer).unwrap();

    if buffer.is_empty() {
        return None;
    }

    let bytes: Lockfile = rkyv::from_bytes(&buffer).unwrap();

    if bytes.packages.is_empty() {
        return None;
    }

    for package in bytes.packages {
        if package.name == name {
            return Some(package);
        }
    }

    None
}
