#!/bin/bash

# Build script for alfred-zed-projects workflow
# Creates a distributable .alfredworkflow file

set -e

echo "Building Alfred Zed Projects workflow..."

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf target/
rm -f alfred-zed-projects.alfredworkflow

# Build the Rust project
echo "Building Rust project..."
cargo build --release

# Copy the binary to the workflow directory
echo "Copying binary to workflow directory..."
cp target/release/alfred-zed workflow/alfred-zed

# Create the .alfredworkflow file (which is just a zip file)
echo "Creating .alfredworkflow file..."
cd workflow
zip -r ../alfred-zed-projects.alfredworkflow .
cd ..

echo "✅ Build complete! Created: alfred-zed-projects.alfredworkflow"
echo "📦 You can now install this workflow by double-clicking the file"