# git-gen

A tool for automatically generating Git commit messages using AI.

## Features

- 🤖 Support for multiple LLM providers (OpenAI, Gemini)
- 📝 Automatically generate conventional commit messages from staged changes
- ⚙️ Flexible configuration (config file or environment variables)
- 🚀 One-click commit (optional)
- 🔧 Built with Rust for excellent performance

## Installation

### Download from Releases

Visit the [Releases](https://github.com/fileng87/git-gen/releases) page and download the binary for your platform:

- Linux: `git-gen-linux-x86_64`
- macOS (Intel): `git-gen-macos-x86_64`
- macOS (Apple Silicon): `git-gen-macos-arm64`
- Windows: `git-gen-windows-x86_64.exe`

After downloading, rename the file to `git-gen` (Windows users keep the `.exe` extension) and add it to your PATH.

**Linux/macOS Example:**

```bash
# Download and rename
mv git-gen-linux-x86_64 git-gen
chmod +x git-gen

# Move to system path
sudo mv git-gen /usr/local/bin/
```

**Windows PowerShell Example:**

```powershell
# Add to current session PATH
$env:PATH += ";C:\path\to\git-gen"

# Or permanently add to user PATH
[Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";C:\path\to\git-gen", "User")
```

### Build from Source

Requires [Rust](https://www.rust-lang.org/tools/install) and Git to be installed.

```bash
# Clone the repository
git clone https://github.com/fileng87/git-gen.git
cd git-gen

# Build
cargo build --release

# Binary will be at target/release/git-gen (Windows: git-gen.exe)
```

## Configuration

### Method 1: Config File (Recommended)

On first run, `git-gen` will automatically create a config file template at `~/.git-gen/config.toml`.

Edit the config file:

```toml
default_provider = "openai"  # Optional: openai or gemini

[openai]
api_key = "your-openai-api-key"
model = "gpt-4o-mini"  # Or other OpenAI models

[gemini]
api_key = "your-gemini-api-key"
model = "gemini-pro"  # Or other Gemini models
```

### Method 2: Environment Variables

You can also configure via environment variables:

**OpenAI:**
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_MODEL="gpt-4o-mini"
```

**Gemini:**
```bash
export GEMINI_API_KEY="your-api-key"
export GEMINI_MODEL="gemini-pro"
```

**Windows PowerShell:**
```powershell
$env:OPENAI_API_KEY = "your-api-key"
$env:OPENAI_MODEL = "gpt-4o-mini"
```

## Usage

### Basic Usage

1. **Stage your changes:**
   ```bash
   git add .
   ```

2. **Generate commit message (preview):**
   ```bash
   git gen commit
   ```

3. **Generate and commit automatically:**
   ```bash
   git gen commit --apply
   ```

### Specify LLM Provider

If you have multiple providers configured, you can specify which one to use with the `--llm` flag:

```bash
# Use OpenAI
git gen commit --llm openai

# Use Gemini
git gen commit --llm gemini

# Generate and commit
git gen commit --llm openai --apply
```

### Command Options

```
git-gen commit [OPTIONS]

Options:
  -a, --apply    Apply the generated commit message and commit changes
  -l, --llm <LLM>  Specify LLM provider (openai, gemini)
  -h, --help     Show help information
```

## Workflow Example

```bash
# 1. Make code changes
vim src/main.rs

# 2. Stage changes
git add src/main.rs

# 3. Generate commit message (preview)
git gen commit
# Output: feat: Add error handling for file operations

# 4. If satisfied, apply the commit
git gen commit --apply
# ✓ Commit applied successfully
```

## Project Structure

```
git-gen/
├── crates/
│   ├── cli/          # Command-line interface
│   ├── core/          # Core business logic
│   ├── git/           # Git operations wrapper
│   └── llm/           # LLM provider implementations
├── .github/
│   └── workflows/     # CI/CD configuration
└── Cargo.toml         # Workspace configuration
```

## Development

### Prerequisites

- Rust 1.70+
- Git

## Contributing

Issues and Pull Requests are welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
