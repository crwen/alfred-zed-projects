# alfred-zed-projects

Alfred workflow that allows you to open projects with [zed](https://github.com/zed-industries/zed)

## Prerequisties

- [jq](https://github.com/jqlang/jq) (you can remove extensions lookup function if you don't want to install this)
- [zed](https://github.com/zed-industries/zed)

## Installation

You can download workflow file, or compile your own program.

## Building from Source

### Prerequisites for Building

- [Rust](https://rustup.rs/) (latest stable version)
- [jq](https://github.com/jqlang/jq) (you can remove extensions lookup function if you don't want to install this)
- [zed](https://github.com/zed-industries/zed)

### Build Instructions

#### Quick Build (Recommended)

1. Clone the repository:
   ```bash
   git clone https://github.com/crwen/alfred-zed-projects.git
   cd alfred-zed-projects
   ```

2. Run the build script:
   ```bash
   ./build-workflow.sh
   ```

3. Install the generated `alfred-zed-projects.alfredworkflow` file by double-clicking it.

#### Manual Build

1. Clone the repository:
   ```bash
   git clone https://github.com/crwen/alfred-zed-projects.git
   cd alfred-zed-projects
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Copy the binary to the workflow directory:
   ```bash
   cp target/release/alfred-zed workflow/alfred-zed
   ```

4. Install the workflow by double-clicking the `workflow` folder or importing it into Alfred.

## Usage

- `zf`: Search file and Open with Zed.
- `zr`: Open recent projects. Data is provided by Zed sqlite file.
- `ze`: Lookup extensions, nothing else. So you can remove it by yourself.
