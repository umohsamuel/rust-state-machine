# 🔄 Rust State Machine

A minimal, type-safe **state machine** implementation written in **Rust**, designed to model predictable and testable transitions between states.

This project demonstrates how Rust’s type system, enums, and pattern matching can be leveraged to build finite state machines with clear and safe transitions.

---

## 🚀 Features

- ✅ Finite state machine (FSM) implementation
- 🔄 Predictable state transitions
- 🧪 Testable and deterministic logic
- 🔍 Transparent transition logging (optional)
- ♻️ Built with Enums and Rust traits
- 📚 Great for modeling workflows, UIs, lifecycles, etc.

---

## 📦 Use Cases

- Task/workflow automation
- UI/UX page or modal states
- Game development logic
- API lifecycle management
- Finite protocol modeling

---

## 📁 Project Structure

```bash
rust-state-machine/
├── src/
│ ├── main.rs # Demo & entry point
│ ├── state.rs # State and transition logic
│ └── lib.rs # (Optional) for library-style use
├── tests/ # Unit tests
├── Cargo.toml # Dependencies and project metadata
└── README.md # Project documentation
```

## 📦 Installation & Running

> Requires [Rust & Cargo](https://www.rust-lang.org/tools/install)

### 1. Clone the repository

```bash
git clone https://github.com/umohsamuel/rust-state-machine.git
cd rust-state-machine
```

### 2. Build and run

```bash
cargo run
```
This runs the main demonstration of the state machine logic.

## 🧪 Running Tests
```bash
cargo test
```
All state transitions and logic validations are covered under unit tests.


## 🔧 Example State Machine
A basic structure might look like:

```bash
enum State {
    Idle,
    Loading,
    Success,
    Error,
}

enum Event {
    Start,
    Finish,
    Fail,
    Reset,
}

```
With transitions like:

Idle + Start → Loading

Loading + Finish → Success

Loading + Fail → Error

Error + Reset → Idle

## To format: 
```bash
cargo +nightly fmt
```

## To expand macros and see whats happening under the hood
```bash
cargo install cargo-expand
cargo expand > out.rs
```
