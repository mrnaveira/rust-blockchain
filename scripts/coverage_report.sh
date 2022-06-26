# This script generates a test coverage report under the "/coverage" folder
# To run it you will need Rust version 1.60 or greater and install the following:
#       $ cargo install grcov
#       $ rustup component add llvm-tools-preview

# Source of the instructions:
# https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project

cargo clean
rm -rf ./coverage ./target *.prof*

# Export the flags needed to instrument the program to collect code coverage.
export RUSTFLAGS="-C instrument-coverage"

# Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable
# (%p will be replaced by the process ID, and %m by the binary signature):
export LLVM_PROFILE_FILE="rust_blockchain-%p-%m.profraw"

# Build the program
cargo build

# Run the program
cargo test

# Generate a HTML report in the coverage/ directory.
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore "tests/*" -o ./coverage/

rm *.prof*