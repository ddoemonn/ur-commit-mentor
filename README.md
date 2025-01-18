# ur-commit-mentor 🔍

A powerful CLI tool that analyzes git commits and provides AI-powered code review insights using Claude AI (currently the only supported AI provider).

## Why Use ur-commit-mentor? 🤔

Ever written code, committed changes, and then wished you had a second pair of eyes before pushing? ur-commit-mentor acts as your AI-powered code review companion:

- **Pre-Push Analysis**: Review your commits before pushing to catch potential issues
- **AI-Powered Insights**: Get intelligent suggestions about code quality, best practices, and potential improvements
- **Quick Feedback**: Understand the impact of your changes without waiting for human review

## Installation 📦

```bash
# Install from crates.io
cargo install ur-commit-mentor
```

## Demo 🎥

https://github.com/user-attachments/assets/5a76fed2-3924-4fd6-924d-a97055ee1b4e

## Features ✨

- Interactive commit selection and fuzzy search
- Detailed code analysis powered by Claude AI 
- Beautiful terminal UI with syntax highlighting
- Language-specific insights
- Visual commit statistics
- Progress bars and visual metrics
- Support for all git repositories

## Prerequisites 🔑

1. [Rust and Cargo](https://rustup.rs/) installed on your system
2. A Claude API key from [Anthropic](https://www.anthropic.com/)
3. Git repository to analyze

## Usage 💻

```bash
# Basic usage
ur-commit-mentor <repository_path> <claude_api_key>

# Example
ur-commit-mentor ./my-project "sk-ant-api03-xxxx..."

# With relative path
cd ~/projects
ur-commit-mentor ./awesome-project "your-api-key"
```

## Contributing 🤝

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author ✍️

Özer Gökalpsezer - [@ddoemonn](https://github.com/ddoemonn)

## Acknowledgments 🙏

- [Claude AI](https://www.anthropic.com/) for powering the code analysis
- The Rust community for amazing libraries
