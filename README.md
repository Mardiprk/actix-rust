# Actix Web Introduction

<div align="center">
  <img src="https://miro.medium.com/v2/resize:fit:1000/0*gYeEWqnWqw8P-yuF.png" alt="Actix Logo" width="300"/>
</div>

A simple Actix web server demonstrating basic HTTP endpoints and JSON handling.

## Features

- **Hello World**: Simple GET endpoint returning a greeting
- **Dynamic Routes**: Path parameters for personalized responses
- **JSON API**: POST endpoint that accepts and returns JSON data
- **Async/Await**: Modern Rust async programming patterns

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Returns "Hello, Actix" |
| GET | `/about/{name}` | Returns personalized message about Actix |
| POST | `/greet` | Accepts JSON with name, returns greeting |

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd actix-intro
```

2. Run the server:
```bash
cargo run
```

The server will start at `http://localhost:8080`

### Usage Examples

#### Hello World
```bash
curl http://localhost:8080/
# Response: "Hello, Actix"
```

#### Dynamic Route
```bash
curl http://localhost:8080/about/Alice
# Response: "Alice wants to know about Actix"
```

#### JSON API
```bash
curl -X POST http://localhost:8080/greet \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob"}'
# Response: {"message": "Hello, Bob!"}
```

## Project Structure

```
actix-intro/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
└── README.md
```

## Dependencies

- `actix-web`: Web framework
- `serde`: Serialization/deserialization
- `tokio`: Async runtime

## License

This project is open source and available under the [MIT License](LICENSE). 