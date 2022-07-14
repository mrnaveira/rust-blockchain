# This script generates a test coverage report under the "/coverage" folder
# To run it you will need Rust version 1.60 or greater and install the following:
#       $ cargo install grcov
#       $ rustup component add llvm-tools-preview

# Source of the instructions:
# https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project

cargo clean
rm -rf ./coverage ./target *.prof* */*.prof*

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
grcov . --binary-path ./target/debug/ -s . -t html --branch -o ./coverage/ --ignore-not-existing \
    --ignore "*/main.rs" \ # the "main.rs" is kept minimal, everything relevant is exported (and tested) on "lib.rs"
    --ignore "*/cli.rs" \ # hard/irrelevant to test as it's only used on "main.rs"
    --ignore "*/tests/*" # the coverage on integration test code itself does not make sense

rm *.prof* */*.prof*