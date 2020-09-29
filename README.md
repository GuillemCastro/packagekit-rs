# packagekit-rs
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGuillemCastro%2Fpackagekit-rs.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FGuillemCastro%2Fpackagekit-rs?ref=badge_shield)


PackageKit bindings and wrappers for Rust. packagekit-sys is the sys-level (unsafe, using FFI) crate automatically generated using [gtk-rs/gir](https://github.com/gtk-rs/gir). packagekit-rs is the safe Rust wrapper build on top the sys crate.

The objective of this crate isn't to create a 1:1 version of the original PackageKit's library, but to create a Rust library for interacting with PackageKit.

## Usage

I recommend using the safe Rust wrapper, but not everything is supported. For how to use the sys crate, you can read the [official docs for `packagekit-glib2`](https://www.freedesktop.org/software/PackageKit/gtk-doc/lpackagekit-glib2.html).

For using the wrapper, see [this example](examples/example.rs).

## License

packagekit-rs is licensed under the MIT license. See [LICENSE](LICENSE)

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGuillemCastro%2Fpackagekit-rs.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FGuillemCastro%2Fpackagekit-rs?ref=badge_large)