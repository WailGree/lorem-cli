# Lorem CLI

A Rust CLI tool for generating lorem ipsum text with customizable units and counts.

## Features

- Generate lorem ipsum text by words, sentences, or paragraphs
- Customizable count for each unit type
- Default behavior: generates one sentence
- Clean, modern CLI interface with help and version flags

## Installation

### Prebuilt Binaries (GitHub Releases)

Download the latest release for your platform from the [GitHub Releases page](https://github.com/XavierFabregat/lorem-cli/releases):

- **Linux:** `lorem-cli-linux-amd64`
- **macOS (Intel):** `lorem-cli-macos-amd64`
- **macOS (Apple Silicon):** `lorem-cli-macos-arm64`
- **Windows:** `lorem-cli-windows-amd64.exe`

1. Download the appropriate binary for your OS and architecture.
2. (Linux/macOS) Make it executable:
   ```bash
   chmod +x lorem-cli-*
   ```
3. (Optional) Move it to a directory in your PATH, e.g.:
   ```bash
   sudo mv lorem-cli-linux-amd64 /usr/local/bin/lorem-cli
   ```

### Using Cargo (crates.io)

Install directly from [crates.io](https://crates.io/crates/lorem-cli):

```bash
cargo install lorem-cli
```

### From Source

1. Clone the repository
2. Navigate to the project directory
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be available at `target/release/lorem-cli`

## Usage

### Basic Usage

Generate one sentence (default):

```bash
lorem-cli
```

### Command Line Options

- `-u, --unit <UNIT>`: Type of unit to count. Valid values:

  | Long      | Short |
  | --------- | ----- |
  | word      | w     |
  | sentence  | s     |
  | paragraph | p     |

  [default: sentence]

- `-c, --count <COUNT>`: Number of units to generate [default: 1]
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Examples

Generate 5 words:

```bash
lorem-cli --unit word --count 5
lorem-cli --unit w --count 5
```

Generate 3 sentences:

```bash
lorem-cli --unit sentence --count 3
lorem-cli --unit s --count 3
```

Generate 2 paragraphs:

```bash
lorem-cli --unit paragraph --count 2
lorem-cli --unit p --count 2
```

Using short flags:

```bash
lorem-cli -u word -c 10
lorem-cli -u w -c 10
lorem-cli -u sentence -c 2
lorem-cli -u s -c 2
lorem-cli -u paragraph -c 1
lorem-cli -u p -c 1
```

## Output Examples

### Words

```
lorem ipsum dolor sit amet
```

### Sentences

```
lorem ipsum dolor sit amet consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
```

### Paragraphs

```
lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum.

sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt neque porro quisquam est qui dolorem ipsum quia dolor sit amet consectetur adipisci velit sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.
```

## Dependencies

- `clap`: Command line argument parsing
- `anyhow`: Error handling
- `fastrand`: Fast random number generation

## License

This project is open source and available under the MIT OR Apache-2.0 License.
