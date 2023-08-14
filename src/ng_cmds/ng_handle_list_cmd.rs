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

pub struct ListCmd;

impl NgExecBase for ListCmd {
    fn exec_mode(&self, args: Vec<String>) -> Result<(), NgError> {
        println!("Listing...");

        // Get the package and the group and list all the packages under the group
        let group: &str = &args[0];
        let packages = match ng_package_handler::ng_list_packages(group, false) {
            Ok(pkgs) => {
                if pkgs.len() > 0 {
                    for package in &pkgs {
                        println!("  - {package}");
                    }
                    println!("\nThere are {} packages", pkgs.len());
                } else {
                    println!("There are no packages!");
                }
                Ok(())
            },
            Err(_e)  => Err(NgError),
        };
        packages
    }
}
