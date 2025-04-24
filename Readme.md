# Fly.io Distributed Systems Challenge

Welcome to the **Fly.io Distributed Systems Challenge**! This repository contains solutions and resources for tackling the challenges provided by Fly.io. Below, you'll find instructions for running the first challenge and an overview of the project.

---

## Challenge 1: Echo

The first challenge involves creating a simple echo server. The goal is to ensure that the server responds with the same message it receives. This challenge can be executed using the `maelstrom` testing tool.

### Running Challenge 1

To execute Challenge 1, use the following command:

```bash
./maelstrom/maelstrom test -w echo --bin ./echo/target/debug/echo --node-count 1 --time-limit 10
```

### Prerequisites

Ensure you have the following installed and configured:

- **Rust**: The echo server is written in Rust. Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Maelstrom**: A testing tool for distributed systems. Download it from the [official Maelstrom repository](https://github.com/jepsen-io/maelstrom).

### Steps to Run

1. Clone this repository:
    ```bash
    git clone https://github.com/your-username/flyio-dist-sys-challenge.git
    cd flyio-dist-sys-challenge
    ```

2. Build the echo server:
    ```bash
    cd echo
    cargo build --release
    ```

3. Run the Maelstrom test:
    ```bash
    ./maelstrom/maelstrom test -w echo --bin ./echo/target/debug/echo --node-count 1 --time-limit 10
    ```

---

## Project Structure

```plaintext
flyio-dist-sys-challenge/
├── echo/                  # Echo server implementation
│   ├── src/               # Source code for the echo server
│   └── Cargo.toml         # Rust project configuration
├── maelstrom/             # Maelstrom testing tool
└── Readme.md              # Project documentation
```

---

## Resources

- [Fly.io Distributed Systems Challenges](https://fly.io/dist-sys/)
- [Maelstrom Documentation](https://github.com/jepsen-io/maelstrom)
- [Rust Programming Language](https://www.rust-lang.org/)

---
<!-- 
## License

This project is licensed under the [MIT License](LICENSE).

Happy coding and good luck with the challenges! 
-->