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

use std::env;

mod ng_arg_handler;
mod ng_exec_handler;
mod ng_package_handler;
mod ng_error;
mod ng_cmds;

const NUGLOBAL_VERSION: &str = "0.1.0";

fn main() {
    // Show version
    println!("NuGlobal version {NUGLOBAL_VERSION}");
    println!("Copyright (c) 2023 Aptivi");

    // Handle arguments
    let ng_args: Vec<String> = env::args().skip(1).collect();
    let mut ng_mode = "";
    let mut ng_mode_args: Vec<String> = vec![];
    let mut ng_args_satisfied: bool = false;
    dbg!(&ng_args);
    dbg!(&ng_args.len());
    if ng_args.len() > 0 {
        // Parse the argument mode.
        let ng_modes = ng_arg_handler::NG_AVAILABLE_MODES;
        let ng_args_mode: &str = &ng_args[0];
        dbg!(&ng_args_mode);

        // Check to see if the mode is found within the available modes
        for ng_mode_t in ng_modes {
            let ng_mode_name = ng_mode_t.0;
            let ng_mode_req_args = ng_mode_t.1;
            dbg!(&ng_mode_name, &ng_mode_req_args);

            // Check for equality
            if ng_args_mode == ng_mode_name {
                // Now, check for the number of provided arguments
                let ng_env_mode_args: Vec<String> = env::args().skip(2).collect();
                dbg!(&ng_env_mode_args);
                dbg!(&ng_env_mode_args.len(), &ng_mode_req_args);
                if ng_env_mode_args.len() >= ng_mode_req_args.try_into().unwrap() {
                    ng_args_satisfied = true;

                    // Install the results
                    ng_mode = ng_mode_name;
                    ng_mode_args = ng_env_mode_args;
                }
            }
        }
    }

    // Handle execution
    dbg!(&ng_args_satisfied, &ng_mode, &ng_mode_args);
    if ng_args_satisfied {
        let handle_exec = || -> Result<(), ng_error::NgError> {
            ng_exec_handler::handle_exec(ng_mode, ng_mode_args)?;
            Ok(())
        };

        if let Err(_err) = handle_exec() {
            println!("An error occurred while performing a NuGlobal operation: {_err}")
        }
    }
    else {
        let args: Vec<String> = env::args().collect();
        println!("Required arguments are not provided or mode not found. Help usage:\n");
        println!("- {} install <group> <package>", &args[0]);
        println!("    Adds a nupkg package to a group found in the per-user directory source (default is /etc/ng/packages/<group>)\n");
        println!("- {} install-systemwide <group> <package>", &args[0]);
        println!("    Adds a nupkg package to a group found in the systemwide directory source (default is $HOME/.config/ng/packages/<group>/)\n");
        println!("- {} uninstall <group> <package>", &args[0]);
        println!("    Removes a nupkg package from the per-user directory source\n");
        println!("- {} uninstall-systemwide <group> <package>", &args[0]);
        println!("    Removes a nupkg package from the systemwide directory source\n");
        println!("- {} list <group>", &args[0]);
        println!("    Lists installed nupkg packages in the per-user directory source\n");
        println!("- {} list-systemwide <group>", &args[0]);
        println!("    Lists installed nupkg packages in the systemwide directory source\n");
        println!("- {} init <group> <target>", &args[0]);
        println!("    Copies all the packages from the group to a target directory\n");
        println!("- {} init-systemwide <group> <target>", &args[0]);
        println!("    Copies all the packages from the systemwide group to a target directory\n");
    }
}
