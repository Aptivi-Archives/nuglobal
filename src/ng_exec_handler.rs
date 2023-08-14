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

use crate::ng_error::NgError;
use crate::ng_cmds::*;

pub trait NgExecBase {
    fn exec_mode(&self, args: Vec<String>) -> Result<(), NgError>;
}

pub struct UnimplementedCmd;

impl NgExecBase for UnimplementedCmd {
    fn exec_mode(&self, _args: Vec<String>) -> Result<(), NgError> {
        Err(NgError)
    }
}

pub fn handle_exec(mode: &str, args: Vec<String>) -> Result<(), NgError> {
    let exec_base = resolve_exec(mode);
    exec_base.exec_mode(args)
}

pub fn resolve_exec(mode: &str) -> Box<dyn NgExecBase> {
    match mode {
        "install"               => return Box::new(ng_handle_install_cmd::InstallCmd),
        "uninstall"             => return Box::new(ng_handle_uninstall_cmd::UninstallCmd),
        "list"                  => return Box::new(ng_handle_list_cmd::ListCmd),
        "init"                  => return Box::new(ng_handle_init_cmd::InitCmd),
        "install-systemwide"    => return Box::new(ng_handle_install_systemwide_cmd::InstallSystemwideCmd),
        "uninstall-systemwide"  => return Box::new(ng_handle_uninstall_systemwide_cmd::UninstallSystemwideCmd),
        "list-systemwide"       => return Box::new(ng_handle_list_systemwide_cmd::ListSystemwideCmd),
        "init-systemwide"       => return Box::new(ng_handle_init_systemwide_cmd::InitSystemwideCmd),
        _                       => return Box::new(UnimplementedCmd),
    }
}
