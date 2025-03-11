# DLTUI - DLT Log Viewer

A terminal-based user interface (TUI) tool for viewing and analyzing Covesa DLT (Diagnostic Log and Trace) log files.

## Features

- **File Management**: Open and navigate between multiple DLT log files
- **Message Viewing**: Browse through DLT messages with a clean, organized interface
- **Search Functionality**: Search through log messages with regex support
  - Highlight matching text in messages
  - Navigate between search results with keyboard shortcuts
- **Filtering**: Filter messages based on various criteria
- **Detail View**: Examine individual messages in detail
- **Keyboard Navigation**: Efficient keyboard-based interface

## Installation

### Prerequisites

- Rust and Cargo (1.56.0 or later)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/dltui.git
cd dltui

# Build the project
cargo build --release

# The binary will be available at target/release/dltui
```

## Usage

```bash
# Basic usage
dltui [OPTIONS] [FILE]...

# Open specific DLT files
dltui path/to/file1.dlt path/to/file2.dlt

# Open a file with a search pattern
dltui -s "error" path/to/file.dlt

# Open a file with a filter
dltui -f "app_id=APP1" path/to/file.dlt

# For more options
dltui --help
```

## Keyboard Shortcuts

| Key       | Action                     |
|-----------|----------------------------|
| `q`       | Quit                       |
| `/`       | Enter search mode          |
| `n`       | Next search result         |
| `N`       | Previous search result     |
| `f`       | Enter filter mode          |
| `h` or `?`| Show help                  |
| `Enter`   | Toggle detail view         |
| `↑` or `k`| Move up                    |
| `↓` or `j`| Move down                  |
| `g`       | Go to top                  |
| `G`       | Go to bottom               |
| `p`       | Previous file              |

## Search Functionality

The search feature allows you to find specific text within log messages:

1. Press `/` to enter search mode
2. Type your search pattern (supports regex)
3. Press Enter to execute the search
4. Use `n` and `N` to navigate between search results
5. Search matches are highlighted in the message text

## License

This project is licensed under the MIT License - see the LICENSE file for details.
