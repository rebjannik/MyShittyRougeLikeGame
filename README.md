# MyShittyRougeLikeGame

Project for DD1349

A terminal-based roguelike dungeon crawler built with Rust, utilising Test-Driven Development (TDD) and inspired by D&D 5e combat mechanics.

NOTE! AI has been used to help write these documents. Each document is planned by me, but formatting is done through AI. GitHub Copilot also reviews merge requests for safety concerns.

---

# 🎯 Project Vision

The goal of this project is to implement a robust, maintainable game engine where every mechanic—from procedural map generation to combat modifiers—is verified by a suite of automated tests. By separating data from logic, the engine remains scalable and easy to debug.

---

# 🏗️ Architecture & Planning

This project follows a strict separation of **Data (Structs/Enums)** and **Logic (Modules)**.

- **Roadmap & Tasks:** Tracking progress and sub-issues via the Project Planning Site.
- **Internal Wiki:** For detailed technical breakdowns of the Game, Player, and Entity structures, see our Wiki Home.
- **Documentation:** Technical specifications are maintained in the README and associated Wiki pages.

## Project Structure

```text
src/main.rs        -> Game Loop & Entry Point
src/map_gen/       -> Drunkard's Walk algorithms
src/combat/        -> D&D Math, Accuracy, & Defense rolls
src/input/         -> Keyboard event translation
src/renderer/      -> Terminal display & Buffer logic
src/models/        -> Data definitions (Player, Map, Entity)
```

---

# 🚀 Key Features

## Procedural Generation
Cave systems created via Drunkard's Walk algorithms.

## D&D Combat
A two-phase "Attack & Defence" system using 1d20 accuracy checks and attribute-based modifiers.

## Entity System
A unified Rust Enum system handling Monsters, Chests, and Hazards within a single vector.

## TDD-First
100% logic coverage for all core math and state transitions.

---

# 🛠️ Development Setup

## Prerequisites

- Rust & Cargo (Latest Stable)

## Installation

1. Clone the repository using the GitHub URL.
2. Navigate into the directory.
3. Build the project using:

```bash
cargo build
```

---

# 🧪 Running Tests (TDD Workflow)

The test suite is the primary tool for verifying game balance and mechanical accuracy.

Tests are located within each module inside specific test blocks.

Run all tests using:

```bash
cargo test
```

---

# ▶️ Running the Game

Launch the application using:

```bash
cargo run
```

---

# 🎲 Combat Logic Flow

The game utilises a turn-based system where every action is a dice roll.

## Phase 1: The Attack Roll (Player's Turn)

### Accuracy Check
Roll `1d20 + modifiers` vs. Target AC.

### Damage Calculation
If hit, roll weapon dice + modifiers.

### Health Update
Subtract damage from the target's `current_hp`.

---

## Phase 2: The Defence Roll (Monster's Turn)

### Accuracy Check
Monster rolls `1d20 + modifiers` vs. Player AC.

### Damage Calculation
If hit, roll monster damage dice + modifiers.

### Health Update
Subtract damage from the player's `current_hp`.

---

# 📜 License

This project is licensed under the MIT License.
