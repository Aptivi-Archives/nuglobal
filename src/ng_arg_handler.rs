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

pub static NG_AVAILABLE_MODES: &'static [(&str, i32)] = &[
    ("install", 2),
    ("uninstall", 2),
    ("list", 1),
    ("init", 2),
    ("install-systemwide", 2),
    ("uninstall-systemwide", 2),
    ("list-systemwide", 1),
    ("init-systemwide", 2),
];

