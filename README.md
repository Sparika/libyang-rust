# libyang-rust
The goal of this application is mainly to provide a simple template describing the use of libyang with the Rust language.
As in other Rust projects that use C libraries a wrapper "wrapper.h" file is used to include all the required C headers.
The wrapper header is then used by bindgen to generate Rust bindings from the header from build.rs.

The example application simply opens and parses a YANG file.

# Installation
Make sure that rustc and cargo are installed, then run `cargo build`.
Once the project is build it can be started with `cargo run [yang_module_file]`
