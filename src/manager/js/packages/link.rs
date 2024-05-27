use std::fs::create_dir;
use std::path::Path;

use simple_home_dir::home_dir;

use crate::manager::js::obtain::Package;

pub fn link_package(package: &Package) {
    let home_root = home_dir().unwrap();
    let package_dir = format!(
        "{}/.qipi/cache/{}/{}",
        home_root.display(),
        package.name,
        package.version
    );
    if !Path::new(&package_dir).exists() {
        return;
    }

    if !Path::new("./node_modules").exists() {
        create_dir("./node_modules").unwrap();
    }
    
    let dst_path = format!("./node_modules/{}", package.name);
    symlink::symlink_dir(&package_dir, dst_path).unwrap();
}
