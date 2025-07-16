# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Alfred workflow written in Rust that integrates with the Zed editor. The workflow provides three main commands:
- `zf`: Search files and open with Zed
- `zr`: Open recent projects (reads from Zed's SQLite database)
- `ze`: Lookup Zed extensions

## Build and Development Commands

### Building the Project
```bash
cargo build --release
```

### Running/Testing
```bash
cargo run
cargo run "search_query"
```

### Development Build
```bash
cargo build
```

## Code Architecture

### Core Components
- **main.rs**: Single-file application containing all logic
- **Workspace struct**: Represents a workspace from Zed's database
- **Item struct**: Alfred workflow item format for JSON output
- **Icon struct**: File icon configuration for Alfred items

### Key Functionality
- **Database Integration**: Reads from Zed's SQLite database at `~/.config/Zed/db/0-stable/db.sqlite`
- **Query Processing**: Filters workspaces based on command-line arguments
- **Alfred Output**: Serializes results as JSON for Alfred consumption

### Data Flow
1. Connects to Zed's SQLite database
2. Queries workspaces table for recent projects
3. Converts workspace data to Alfred item format
4. Filters results based on user query
5. Outputs JSON to stdout for Alfred

## Alfred Workflow Configuration

The workflow defines three script filters in `workflow/info.plist`:
- **zf**: File search using Alfred's file filter
- **zr**: Recent projects using the compiled `alfred-zed` binary
- **ze**: Extensions lookup using `jq` to parse Zed's extensions index

## Dependencies

External dependencies required:
- **jq**: Required for the `ze` command (extensions lookup)
- **Zed editor**: Must be installed for the workflow to function

Rust crate dependencies:
- `dirs`: For accessing config directories
- `serde`: JSON serialization
- `serde_json`: JSON handling
- `sqlite`: Database access

## Release Configuration

The project uses aggressive optimization settings in `Cargo.toml`:
- Link-time optimization enabled
- Minimal binary size (`opt-level = 'z'`)
- Symbol stripping for smaller binaries
- Panic abort for reduced size