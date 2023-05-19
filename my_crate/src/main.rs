// Release profiles allow us to configure how our code is compiled.
// There are two main profiles in Rust, Dev and Release.
// - Dev --> Defined with good defaults for development
// - Release --> Defined with good defaults for a release build

// If we run 'cargo build' our code will compile with the dev profile.
// We can compile it with the release profile like so: 'cargo build --release'.

// We can configure a lot of default settings for these profiles in the Cargo.toml file.

fn main() {
    println!("Hello, world!");
}
