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

pub struct InitCmd;

impl NgExecBase for InitCmd {
    fn exec_mode(&self, args: Vec<String>) -> Result<(), NgError> {
        println!("Initializing...");

        // Get the group and initialize them to the appropriate path
        let group: &str = &args[0];
        let target: &str = &args[1];
        let status = ng_package_handler::ng_init_packages(group, target, false);
        let mut faulted: bool = false;
        match status {
            Ok(()) => println!("Successfully initialized!"),
            Err(_e) => {
                faulted = true;
                println!("Failed to initialize.");
            },
        };
        if faulted {
            Err(NgError)
        } else {
            Ok(())
        }
    }
}
