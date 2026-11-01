//! The bindings are generated from the XML by a proc macro, which cargo does not
//! track: rebuild them whenever a protocol changes.

fn main() {
    println!("cargo:rerun-if-changed=protocols");
}
