# PerformanceController

A Rust-based system performance monitoring tool with an AI-powered chat interface. Monitor your CPU, GPU, and RAM metrics through an interactive terminal UI and get intelligent performance insights.

## Features

- 📊 Real-time GPU monitoring via NVIDIA Management Library (NVML)
- 💬 AI-powered chat for performance analysis and recommendations
- 🖥️ Interactive terminal UI built with Ratatui
- 📝 Structured logging with configurable verbosity

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- NVIDIA GPU(Future plans cover non-NVIDIA guys) + CUDA Toolkit (for GPU monitoring)
- [Ollama](https://ollama.com) installed locally (for chat)
- A local Ollama model pulled (for example: `llama3.2`)


## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/Riad374-code/PerformanceController.git
cd PerformanceController

# 2. Create your .env file
cp .env.example .env   # or create it manually (see Configuration)

# 3. Start Ollama and pull a model (first time only)
ollama serve
ollama pull llama3.2

# 4. Build and run
cargo run --release
```

That's it! The application will start and send chat requests to your local Ollama instance.

## Configuration

Create a `.env` file in the project root:

```env
OLLAMA_BASE_URL="http://127.0.0.1:11434"
OLLAMA_MODEL="llama3.2"
```

| Variable    | Description                                  | Default          |
|-------------|----------------------------------------------|------------------|
| `OLLAMA_BASE_URL` | Base URL of your Ollama server      | `http://127.0.0.1:11434` |
| `OLLAMA_MODEL`    | Model name used for chat             | `llama3.2` |
| `model`           | Legacy fallback key for model name   | `llama3.2` |

> Chat requests now use Ollama's `/api/chat` endpoint.

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

This project is licensed under the [MIT License](LICENSE).
