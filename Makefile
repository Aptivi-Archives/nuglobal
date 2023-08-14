#
#    NuGlobal  Copyright (C) 2018-2023  Aptivi
# 
#    NuGlobal is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    NuGlobal is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

OUTPUTS = target debian/nuglobal
BINARIES = target/release/nuglobal
MANUALS = doc/nuglobal.1

.PHONY: all all-online-release debian-install

# General use

all: all-online

all-online:
	cargo build

all-online-release:
	cargo build --release

# Below targets are for Debian packaging only

debian-all-offline:
	cargo build --release --offline

debian-install:
	mkdir -m 755 -p debian/nuglobal/usr/bin
	install -m 755 -t debian/nuglobal/usr/bin/ $(BINARIES)
	install -m 755 -t debian/ $(MANUALS)

clean:
	rm -rf $(OUTPUTS)

