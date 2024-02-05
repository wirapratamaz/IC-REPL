# IC REPL Rust Project

This repository contains a simple Rust project for an IC REPL (Internet Computer Read-Eval-Print Loop). Follow the steps below to set up the environment and test commands.

## Step 1: Setting up Rust Environment and Dependencies

Ensure Rust is installed, and Cargo (Rust's package manager and build system) is available. Then, set up a new Rust project:

```bash
cargo new ic_repl
cd ic_repl
```

## Step 2: Running the Project
To run the project, use the following command:
```bash
cargo run
```

## Step 3: Sending Commands
Type the command or query you want to test in the IC REPL prompt, and press Enter. For example:
```bash
IC REPL> Hello, canister!
```
Based on the basic implementation, the system will respond with a format like:
```bash
Result: Command received: Hello, canister!
```

## NOTES
Example Commands
You can try the following commands:

bash
Copy code
```bash
IC REPL> Hello, canister!
IC REPL> Testing 123
IC REPL> Custom command
```

