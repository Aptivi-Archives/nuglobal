/*
 * NuGlobal  Copyright (C) 2023  Aptivi
 *
 * This file is part of NuGlobal
 *
 * NuGlobal is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * NuGlobal is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env::consts::OS;
use std::env::var;
use std::fs;
use std::path::Path;
use crate::ng_error::NgError;

pub fn ng_get_packages_path_systemwide(group: &str) -> Result<String, NgError> {
    let path_match = match OS {
        "linux" => Ok(format!("/usr/share/ng/packages/{group}/")),
        "windows" => match var("ALLUSERSPROFILE") {
            Ok(homedir)  => Ok(format!("{homedir}/NuGlobal/Packages/{group}/")),
            Err(_e)      => Err(NgError),
        }
        &_ => Err(NgError),
    };
    if path_match.is_ok() {
        let resulting_path = path_match.unwrap();
        let create_match = match fs::create_dir_all(&resulting_path) {
            Ok(())  => Ok(resulting_path),
            Err(_e) => Err(NgError),
        };
        return create_match;
    }
    path_match
}

pub fn ng_get_packages_path(group: &str) -> Result<String, NgError> {
    let path_match = match OS {
        "linux" => match var("HOME") {
            Ok(homedir)  => Ok(format!("{homedir}/.config/ng/packages/{group}/")),
            Err(_e)      => Ok(format!("/tmp/.config/ng/packages/{group}/")),
        }
        "windows" => match var("USERPROFILE") {
            Ok(homedir)  => Ok(format!("{homedir}/AppData/Local/NuGlobal/Packages/{group}/")),
            Err(_e)      => Err(NgError),
        }
        &_ => Err(NgError),
    };
    if path_match.is_ok() {
        let resulting_path = path_match.unwrap();
        let create_match = match fs::create_dir_all(&resulting_path) {
            Ok(())  => Ok(resulting_path),
            Err(_e) => Err(NgError),
        };
        return create_match;
    }
    path_match
}

pub fn ng_check_nupkg_path(path_to_nupkg: &str, ng_path: &str) -> Result<String, NgError> {
    // Now, check the path to nupkg
    if Path::new(path_to_nupkg).try_exists().is_err() {
        return Err(NgError);
    }

    // Now, build a path to the package found at the root packages path
    let ng_nupkg_path_unwrapped = Path::new(&ng_path);
    let ng_nupkg_path = ng_nupkg_path_unwrapped.to_str().unwrap();
    if Path::new(ng_nupkg_path).try_exists().is_err() {
        return Err(NgError);
    }
    Ok(String::from(ng_nupkg_path))
}

pub fn ng_check_target_path_for_init(group: &str, target: &str) -> Result<String, NgError> {
    // Check the target path
    if Path::new(target).try_exists().is_err() {
        return Err(NgError);
    } else {
        let target_group = format!("{target}/{group}/");
        let create_match = match fs::create_dir_all(&target_group) {
            Ok(())  => Ok(target_group),
            Err(_e) => Ok(target_group),
        };
        return create_match;
    }
}

pub fn ng_install_package(group: &str, path_to_nupkg: &str, systemwide: bool) -> Result<(), NgError> {
    // Get the root package path
    let mut ng_path: String = String::new();
    let ng_path_result = match systemwide {
        true => ng_get_packages_path_systemwide(group),
        false => ng_get_packages_path(group),
    };
    let ng_path_match = match ng_path_result {
        Ok(path)  => {
            ng_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_path, &path_to_nupkg);
    if ng_path_match.is_err() {
        return ng_path_match;
    }

    // Now, check the path to nupkg
    let mut ng_nupkg_path: String = String::new();
    let ng_nupkg_path_match = match ng_check_nupkg_path(path_to_nupkg, &path_to_nupkg) {
        Ok(path)  => {
            ng_nupkg_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_nupkg_path);
    if ng_nupkg_path_match.is_err() {
        return ng_nupkg_path_match;
    }

    // Prepare the nupkg path
    let ng_nupkg_path_unwrapped = Path::new(&ng_nupkg_path).file_name().unwrap().to_str().unwrap();
    let ng_final_destination_unwrapped = Path::new(&ng_path).join(&ng_nupkg_path_unwrapped);
    let ng_final_destination = ng_final_destination_unwrapped.to_str().unwrap();
    let exists = Path::new(&ng_final_destination).is_file();
    dbg!(&ng_final_destination);
    if exists {
        return Err(NgError);
    }

    // Finally, install the package.
    let result = match fs::copy(ng_nupkg_path, ng_final_destination) {
        Ok(_bytes) => Ok(()),
        Err(_e)    => Err(NgError),
    };
    result
}

pub fn ng_uninstall_package(group: &str, nupkg_name: &str, systemwide: bool) -> Result<(), NgError> {
    // Get the root package path
    let mut ng_path: String = String::new();
    let ng_path_result = match systemwide {
        true => ng_get_packages_path_systemwide(group),
        false => ng_get_packages_path(group),
    };
    let ng_path_match = match ng_path_result {
        Ok(path)  => {
            ng_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_path);
    if ng_path_match.is_err() {
        return ng_path_match;
    }

    // Now, check the path to nupkg
    let mut ng_nupkg_path: String = String::new();
    dbg!(format!("{ng_path}/{nupkg_name}.nupkg").as_str());
    let ng_nupkg_path_match = match ng_check_nupkg_path(&ng_path, format!("{ng_path}/{nupkg_name}.nupkg").as_str()) {
        Ok(path)  => {
            ng_nupkg_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_nupkg_path);
    if ng_nupkg_path_match.is_err() {
        return ng_nupkg_path_match;
    }

    let ng_nupkg_path_unwrapped = Path::new(&ng_nupkg_path).file_name().unwrap().to_str().unwrap();
    let ng_final_destination_unwrapped = Path::new(&ng_path).join(&ng_nupkg_path_unwrapped);
    let ng_final_destination = ng_final_destination_unwrapped.to_str().unwrap();
    let exists = Path::new(&ng_final_destination).is_file();
    dbg!(&ng_final_destination);
    if !exists {
        return Err(NgError);
    }

    // Finally, uninstall the package.
    let result = match fs::remove_file(ng_final_destination) {
        Ok(_bytes) => Ok(()),
        Err(_e)    => Err(NgError),
    };
    result
}

pub fn ng_list_packages(group: &str, systemwide: bool) -> Result<Vec<String>, NgError> {
    // Get the root package path
    let mut ng_path: String = String::new();
    let ng_path_result = match systemwide {
        true => ng_get_packages_path_systemwide(group),
        false => ng_get_packages_path(group),
    };
    let ng_path_match = match ng_path_result {
        Ok(path)  => {
            ng_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_path);
    if ng_path_match.is_err() {
        return Err(NgError);
    }

    // Finally, list the packages.
    let mut ng_packages: Vec<String> = vec![];
    for fs_entry in fs::read_dir(ng_path).unwrap() {
        let fs_path = fs_entry.unwrap().path();
        if fs_path.is_file() && fs_path.extension().unwrap() == "nupkg" {
            ng_packages.resize_with(ng_packages.len() + 1, || fs_path.file_name().unwrap().to_str().unwrap().to_string());
        }
    };
    Ok(ng_packages)
}

pub fn ng_init_packages(group: &str, path_to_target: &str, systemwide: bool) -> Result<(), NgError> {
    // Get the root package path
    let mut ng_path: String = String::new();
    let ng_path_result = match systemwide {
        true => ng_get_packages_path_systemwide(group),
        false => ng_get_packages_path(group),
    };
    let ng_path_match = match ng_path_result {
        Ok(path)  => {
            ng_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_path);
    if ng_path_match.is_err() {
        return ng_path_match;
    }

    // Now, check the path to target
    let mut ng_target_path: String = String::new();
    let ng_target_path_match = match ng_check_target_path_for_init(group, path_to_target) {
        Ok(path)  => {
            ng_target_path = path;
            Ok(())
        },
        Err(_e)   => Err(NgError),
    };
    dbg!(&ng_target_path);
    if ng_target_path_match.is_err() {
        return ng_target_path_match;
    }

    // Finally, initialize the packages.
    let ng_listed_packages = ng_list_packages(group, systemwide);
    let mut ng_failed: Vec<String> = vec![];
    let mut ng_succeeded: Vec<String> = vec![];
    dbg!(&ng_listed_packages);
    for listed in ng_listed_packages.unwrap() {
        let path = format!("{ng_path}{listed}");
        let target = format!("{ng_target_path}{listed}");
        dbg!(&path, &target);
        match fs::copy(&path, &target) {
            Ok(_bytes) => ng_succeeded.resize_with(ng_succeeded.len(), || listed.to_string()),
            Err(_e)    => ng_failed.resize_with(ng_failed.len(), || listed.to_string()),
        };
    }

    // Install NuGet.Config to the target
    let mut ng_parent_target_path: &str = &ng_target_path;
    match Path::new(&ng_target_path).parent() {
        Some(new_path) => ng_parent_target_path = new_path.to_str().unwrap(),
        None => (),
    };
    let ng_nuget_config_contents =
    r#"
<?xml version="1.0" encoding="utf-8"?>
<configuration>
    <packageSources>
        <clear />
        <add key="nuget.org" value="./group" />
    </packageSources>
</configuration>
"#  .replace("group", &group);
    dbg!(&ng_parent_target_path);
    dbg!(&ng_nuget_config_contents);
    let write_result = fs::write(format!("{ng_parent_target_path}/NuGet.config"), ng_nuget_config_contents);

    // If initialization failed, return error
    if ng_failed.len() > 0 || write_result.is_err() {
        Err(NgError)
    } else {
        Ok(())
    }
}
