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

use crate::ng_exec_handler::NgExecBase;
use crate::ng_error::NgError;
use crate::ng_package_handler;

pub struct InstallSystemwideCmd;

impl NgExecBase for InstallSystemwideCmd {
    fn exec_mode(&self, args: Vec<String>) -> Result<(), NgError> {
        println!("Installing...");

        // Get the package and the group and install it to the appropriate path
        let group: &str = &args[0];
        let pkgs: Vec<String> = args.clone().into_iter().skip(1).collect();
        let mut failed_packages: Vec<&str> = vec![];
        let mut installed_packages: Vec<&str> = vec![];
        for package in &pkgs {
            let status = ng_package_handler::ng_install_package(group, &package, true);
            match status {
                Ok(())  => installed_packages.resize_with(installed_packages.len() + 1, || &package),
                Err(_e) => failed_packages.resize_with(failed_packages.len() + 1, || &package),
            }
        }

        // Check to see if there is an error
        if failed_packages.len() > 0 {
            if installed_packages.len() > 0 {
                println!("NuGlobal installed some of the packages.");
            }
            println!("Failed to install below packages:");
            for failed_package in failed_packages {
                println!("  - {failed_package}")
            }
            Err(NgError)
        } else {
            println!("NuGlobal successfully installed all the packages!");
            Ok(())
        }
    }
}
