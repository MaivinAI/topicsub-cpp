include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4 # Optionally specify a commit hash, version tag or branch here
)

# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

cmake_minimum_required(VERSION 3.15)
project(test_project VERSION 0.1.0)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(zenohlib
    CRATE zenohrs
    FILES lib.rs
)
