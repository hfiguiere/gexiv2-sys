gexiv2-sys
==========

[![Build Status](https://img.shields.io/travis/felixc/gexiv2-sys.svg)](https://travis-ci.org/felixc/gexiv2-sys)&nbsp;
[![Crates.io Downloads](https://img.shields.io/crates/d/gexiv2-sys.svg)](https://crates.io/crates/gexiv2-sys)&nbsp;
[![Crates.io Version](https://img.shields.io/crates/v/gexiv2-sys.svg)](https://crates.io/crates/gexiv2-sys)&nbsp;
[![License](https://img.shields.io/crates/l/gexiv2-sys.svg)](https://github.com/felixc/gexiv2-sys/blob/master/LICENSE)&nbsp;


Rust FFI declarations for gexiv2
--------------------------------

This library provides Rust FFI declarations for the [gexiv2][gexiv2] library,
which is a GObject-based wrapper around the [Exiv2][exiv2] library, which
provides read and write access to the Exif, XMP, and IPTC metadata in media
files.

Only FFI declarations are provided here; **for a usable Rust library, consider
the [rexiv2][rexiv2] crate**.

[gexiv2]: https://wiki.gnome.org/Projects/gexiv2
[exiv2]:  http://www.exiv2.org/
[rexiv2]: https://github.com/felixc/rexiv2


Documentation
-------------

Documentation is [available online][gexiv2-sys], but since these are just FFI
bindings, it is minimal.

[gexiv2’s APIs][gexiv2-api] may be a useful reference, along with [Exiv2’s
API docs][exiv2-api].

[gexiv2-sys]: https://felixcrux.com/files/doc/gexiv2_sys/
[gexiv2-api]: https://git.gnome.org/browse/gexiv2/tree/gexiv2/gexiv2-metadata.h
[exiv2-api]:  http://exiv2.org/doc/index.html


Setup & Dependencies
--------------------

Given that it links to gexiv2, and transitively to Exiv2, gexiv2-sys obviously
depends on them. These libraries are not bundled with gexiv2-sys: you will need
to install them separately.

For full instructions on how to get started with gexiv2-sys, including how to
install the prerequisite dependencies, refer to the [`SETUP`](SETUP.md) file.


Optional Features
-----------------

**raw-tag-access**: If you need access to the raw byte values of tags, you can
enable this feature and gain the `gexiv2_metadata_get_tag_raw` function. Note
that the return value of this call is a GLib [`GBytes`][gbytes] object, which
you can convert to a data pointer via GLib’s [`g_bytes_unref_to_data`][unref].

This feature is disabled by default because it introduces a new dependency on
[`glib-sys`][glib-sys], and consequently on the GLib system library.

**xmp-packet-access**: If you need access to the XML-formatted XMP packet, you
can enable this feature. It will add the `gexiv2_metadata_generate_xmp_packet`
and `gexiv2_metadata_get_xmp_packet` calls. Enabling the feature also introduces
a new dependency on the the [`bitflags`][bitflags] crate.

[gbytes]: http://gtk-rs.org/docs/glib_sys/struct.GBytes.html
[unref]: http://gtk-rs.org/docs/glib_sys/fn.g_bytes_unref_to_data.html
[glib-sys]: https://crates.io/crates/glib-sys/
[bitflags]: https://crates.io/crates/bitflags


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
