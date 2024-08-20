#!/bin/bash

# Function to recursively search for Cargo.toml and clean the project
function clean_cargo_projects {
    local dir=$1

    # Find all Cargo.toml files in the directory
    find "$dir" -name "Cargo.toml" | while read -r cargo_toml; do
        # Get the directory containing the Cargo.toml
        project_dir=$(dirname "$cargo_toml")

        echo "Cleaning Cargo project in: $project_dir"

        # Change to the project directory and run cargo clean
        (cd "$project_dir" && cargo clean)
    done
}

# Directory to start searching from
start_dir="."

# Call the function with the starting directory
clean_cargo_projects "$start_dir"
