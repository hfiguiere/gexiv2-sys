gexiv2-sys
==========

Rust FFI declarations for gexiv2
--------------------------------

This library provides Rust FFI declarations for the [gexiv2][gexiv2] library,
which is a GObject-based wrapper around the [Exiv2][exiv2] library, which
provides read and write access to the Exif, XMP, and IPTC metadata in media
files.

Only FFI declarations are provided here; for a usable Rust library, consider
the [`rexiv2`][rexiv2] crate.

[gexiv2]: https://wiki.gnome.org/Projects/gexiv2
[exiv2]:  http://www.exiv2.org/
[rexiv2]: https://github.com/felixc/rexiv2


Documentation
-------------

[gexiv2’s APIs][gexiv2-api] may be a useful reference, along with [Exiv2’s
API docs][exiv2-api].

[gexiv2-api]: https://git.gnome.org/browse/gexiv2/tree/gexiv2/gexiv2-metadata.h
[exiv2-api]:  http://exiv2.org/doc/index.html


Dependencies
------------

Given that it links to gexiv2, and transitively to Exiv2, gexiv2-sys obviously
depends on them. Only the library (e.g. `.so` or `.dll`) files are needed; not
the headers or source code. You can download these dependencies from their
download pages: [Exiv2][exiv2-dl]; [gexiv2][gexiv2-dl].

On a Linux system, you can typically install these dependencies through your
package manager (look for packages with names like “libgexiv2-dev”). Mac OS X
users may also have this option through unofficial package management systems.
Note that to build gexiv2-sys from source you may need not just the library
packages, but the “dev” versions of them as well.

[exiv2-dl]:  http://www.exiv2.org/download.html
[gexiv2-dl]: https://wiki.gnome.org/Projects/gexiv2/BuildingAndInstalling


Contributing
------------

Contributions are gladly accepted, either through GitHub pull requests or by
mailing patches to `felixc@felixcrux.com` (PGP key [8569B6311EE485F8][pgp-key]).

By contributing, you are agreeing to make your contribution available under the
same license terms as the rest of the project.

[pgp-key]: http://hkps.pool.sks-keyservers.net/pks/lookup?op=vindex&search=0x8569B6311EE485F8

Copyright & License
-------------------

The Exiv2 and gexiv2 libraries are both released under the terms of the GNU
General Public License (GPL), and since gexiv2-sys is linked to them, it too
is made available under the terms of the GPL. Specifically:

This program is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <http://www.gnu.org/licenses/>.

Please refer to the [`LICENSE`](LICENSE) file for a full copy of the license.
