# This script generates a test coverage report under the "/coverage" folder
# To run it, at the moment it's required to use the nightly Rust version:
#       $ rustup install nightly
#       $ rustup default nightly
#       $ cargo install grcov
#       $ rustup component add llvm-tools-preview

# All credit to Marco Castelluccio:
# https://github.com/marco-c/rust-code-coverage-sample

cargo clean
rm -rf ./coverage ./target *.prof*

# Export the flags needed to instrument the program to collect code coverage.
export RUSTFLAGS="-Zinstrument-coverage"

# Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable (%p will be replaced by the process ID, and %m by the binary signature):
export LLVM_PROFILE_FILE="rust_blockchain-%p-%m.profraw"

# Build the program
cargo build

# Run the program
cargo test

# Generate a HTML report in the coverage/ directory.
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/

rm *.prof*