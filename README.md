# Rust Async TCP Echo Server & Client

**Rust Async TCP Echo Server & Client** is a hands-on project demonstrating my ability to build **concurrent networked applications** in Rust. The server echoes back any message sent by the client using **async programming with Tokio**, showcasing my understanding of **TCP networking, Rust’s async ecosystem, and client-server architecture**.

## Key Features

- **Async TCP Echo Server:** Handles multiple clients concurrently without blocking, echoing messages back in real-time.
- **Async TCP Echo Client:** Connects to the server and sends messages, receiving responses asynchronously.
- Lightweight and easy to run locally using `localhost`.

## Skills Demonstrated

- Asynchronous programming in Rust (`async`/`await`)
- TCP networking fundamentals
- Rust’s `tokio` runtime for concurrency
- Client-server application design
- Handling non-blocking I/O and real-time communication

# Install Dependencies

- Make sure you have Rust and Cargo installed.
- This project uses Tokio for async networking

## Run the Server

**Open a terminal and run:**

- cargo run --bin server. By default, the server listens on 127.0.0.1:5000.
- Handles multiple clients concurrently using async tasks.

## Run the Client

**Open another terminal and run:**

- cargo run --bin client
- Connects to 127.0.0.1:5000 by default.
- Type messages in the client terminal; the server will echo them back asynchronously.
