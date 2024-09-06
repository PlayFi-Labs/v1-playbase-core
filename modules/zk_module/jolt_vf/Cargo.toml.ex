[package]
name = "jolt_vf"
version = "0.1.0"
edition = "2021"

[dependencies]
ark-ff = { version = "0.4.2", default-features = false }
jolt = {git = "https://github.com/a16z/jolt", rev = "5eb488381d383034ea1dd558011da0055c5b0c55", package = "jolt-sdk", features = ["guest-std"]}

[lib]
name = "jolt_vf"
path = "src/lib.rs"