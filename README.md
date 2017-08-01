# gstreamer-rs

[GStreamer](https://gstreamer.freedesktop.org/) bindings for Rust.

These bindings are providing a safe API that can be used to interface with
GStreamer, e.g. for writing GStreamer-based applications.

For background and motivation, see the [announcement blogpost](https://coaxion.net/blog/2017/07/writing-gstreamer-applications-in-rust/).

The bindings are autogenerated with [gir](https://github.com/gtk-rs/gir/)
based on the [GObject-Introspection](https://wiki.gnome.org/Projects/GObjectIntrospection/)
API metadata provided by the GStreamer project.

A crate for writing GStreamer plugins in Rust can be found here: https://github.com/sdroege/gst-plugin-rs

## LICENSE

gstreamer-rs and all crates contained in here are licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

GStreamer itself is licensed under the Lesser General Public License version
2.1 or (at your option) any later version:
https://www.gnu.org/licenses/lgpl-2.1.html

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gstreamer-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
