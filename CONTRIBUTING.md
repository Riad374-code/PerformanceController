# Contributing to PerformanceController

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- NVIDIA GPU + CUDA Toolkit (for GPU monitoring features)
- An AI API key (OpenAI-compatible endpoint)

## Getting Started

### 1. Fork the Repository

Click the **Fork** button on the [repository page](https://github.com/Riad374-code/PerformanceController) to create your own copy.

### 2. Clone Your Fork

```bash
git clone https://github.com/<your-username>/PerformanceController.git
cd PerformanceController
```

### 3. Add the Upstream Remote

```bash
git remote add upstream https://github.com/Riad374-code/PerformanceController.git
```

### 4. Set Up the Project

```bash
# Copy the environment file and fill in your values
cp .env.example .env   # or create .env manually

# Build the project
cargo build
```

The `.env` file requires at minimum:

```env
AI_API_KEY="your-api-key-here"
model="gemini-3-flash"
```

### 5. Run the Project

```bash
cargo run --release
```

To enable verbose logging:

```bash
RUST_LOG=debug cargo run --release
```

## Making Changes

### Create a Feature Branch

Always branch off from the latest `main`:

```bash
git fetch upstream
git checkout -b feat/my-feature upstream/main
```

Use a descriptive branch name such as `feat/add-cpu-chart`, `fix/gpu-init-error`, or `docs/update-readme`.

### Make Your Changes

- Keep commits small and focused.
- Write clear commit messages (e.g. `fix: handle NVML init failure gracefully`).
- Run the tests and build before pushing:

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### Push and Open a Pull Request

```bash
git push origin feat/my-feature
```

Then open a Pull Request against the `main` branch on GitHub. In your PR description:

- Summarize **what** changed and **why**.
- Reference any related issues (e.g. `Closes #42`).
- Include steps to test your changes if applicable.

## Keeping Your Fork Up to Date

```bash
git fetch upstream
git rebase upstream/main
```

## Code Style

- Follow standard Rust conventions (`rustfmt`, `clippy`).
- Prefer descriptive variable and function names.
- Add doc comments (`///`) to public items.

## Reporting Issues

Open an issue on GitHub and include:

- A clear title and description.
- Steps to reproduce (if it's a bug).
- Your OS, Rust version (`rustc --version`), and GPU model.

## License

By contributing, you agree that your contributions will be released under the same license as the project.
