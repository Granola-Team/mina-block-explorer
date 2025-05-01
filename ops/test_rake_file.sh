#!/bin/bash

# Exit on any command failure
set -e

rake clean                  # Clean the repo of built artifacts
rake audit                  # Audit the Rust code with cargo-audit
rake build_docs             # Build documentation in the build directory
rake check                  # Use 'cargo check' to verify buildability
rake deploy_mina_indexer    # Deploy mina-indexer
rake shutdown_mina_indexer  # Shut down mina-indexer
rake dev_build              # Build the dev version for front-end WASM bundle
rake format                 # Format the source code
rake jest_test              # Run the Jest tests
rake lint                   # Lint all source code
rake lint_fix               # Fix linting errors
rake lint_ruby              # Lint the Ruby code
rake lint_rust              # Lint the Rust code
rake print_tree             # Print all tasks and their dependencies as a tree
rake release_build          # Build the release version for front-end WASM bundle
rake rust_test              # Test the Rust code
rake test_unit              # Run the unit tests
rake tier1                  # Run the Tier1 tests
rake tier2                  # Invoke the Tier2 regression suite (non-interactive)

echo "All tasks completed successfully!"
