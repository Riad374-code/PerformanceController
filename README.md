# PerformanceController

A Rust-based system performance monitoring tool with an AI-powered chat interface. Monitor your CPU, GPU, and RAM metrics through an interactive terminal UI and get intelligent performance insights.

## Features

- 📊 Real-time GPU monitoring via NVIDIA Management Library (NVML)
- 💬 AI-powered chat for performance analysis and recommendations
- 🖥️ Interactive terminal UI built with Ratatui
- 📝 Structured logging with configurable verbosity

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- NVIDIA GPU + CUDA Toolkit (for GPU monitoring)
- An AI API key (OpenAI-compatible endpoint)

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/Riad374-code/PerformanceController.git
cd PerformanceController

# 2. Create your .env file
cp .env.example .env   # or create it manually (see Configuration)

# 3. Build and run
cargo run --release
```

That's it! The application will start and connect to the configured AI endpoint.

## Configuration

Create a `.env` file in the project root:

```env
AI_API_KEY="your-api-key-here"
model="gemini-3-flash"
```

| Variable    | Description                                  | Default          |
|-------------|----------------------------------------------|------------------|
| `AI_API_KEY` | Bearer token for the AI API               | *(required)*     |
| `model`      | Model name used for chat completions      | `gemini-3-flash` |

> The AI endpoint is currently configured to `http://127.0.0.1:8045/v1`. Update `tui/chat.rs` to point to a different host if needed.

## Integration Guide

### Using the AI chat in your own code

```rust
use performance_controller::tui::chat::{chat_ai, Message, Role};

#[tokio::main]
async fn main() {
    let msg = Message { message: "What is my GPU usage?".into(), role: Role::User };
    let history = vec![];

    match chat_ai(msg, history).await {
        Ok((response, _history)) => println!("AI: {response}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

### Using GPU monitoring

```rust
use performance_controller::component_details::gpu::{init_nvml, gpu_detailing};

fn main() {
    let nvml = init_nvml().expect("Failed to initialize NVML");
    let device = gpu_detailing(&nvml).expect("Failed to get GPU device");
    println!("GPU: {:?}", device.name());
}
```

### Enabling structured logging

```rust
use performance_controller::logging_init::init_logging;

fn main() {
    init_logging();
    // logs are now available via the `tracing` macros
}
```

Set the `RUST_LOG` environment variable to control log verbosity:

```bash
RUST_LOG=debug cargo run --release
```

## Running with Docker

```bash
docker build -t performance-controller .
docker run --gpus all performance-controller
```

> The `--gpus all` flag is required for NVML / GPU monitoring.

## Project Structure

```
src/
├── main.rs                  # Application entry point
├── lib.rs                   # Library root
├── logging_init.rs          # Logging setup
├── component_details.rs     # System component module
├── component_details/
│   └── gpu.rs               # GPU monitoring (NVML)
├── tui.rs                   # TUI module router
└── tui/
    ├── design.rs            # Terminal UI layout & widgets
    └── chat.rs              # AI chat integration
```

## License

This project is currently unlicensed. See the repository for more details.
