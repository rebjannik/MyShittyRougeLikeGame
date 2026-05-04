# MyShittyRougeLikeGame
**Project for DD1349**

A terminal-based roguelike dungeon crawler built with **Rust**, utilizing **Test-Driven Development (TDD)** and inspired by **D&D 5e combat mechanics**.

## 🎯 Project Vision
The goal of this project is to implement a robust, maintainable game engine where every mechanic—from map generation to combat modifiers—is verified by a suite of automated tests.

## 🏗️ Architecture & Planning
This project follows a strict separation of **Data** (Structs/Enums) and **Logic** (Modules).
* **Wiki:** For detailed technical breakdowns, see our [Internal Wiki](https://github.com/rebjannik/MyShittyRougeLikeGame/wiki).
* **Roadmap:** Tracking progress via [GitHub Projects](https://github.com/rebjannik/MyShittyRougeLikeGame/projects).

## 🚀 Key Features
- **Procedural Generation:** Cave systems created via Drunkard's Walk algorithms.
- **D&D Combat:** Accuracy and Damage rolls based on character stats and modifiers.
- **Entity System:** Unified handling of Monsters, Chests, and Hazards using Rust Enums.
- **TDD-First:** 100% logic coverage ensuring game balance and stability.

## 🛠️ Development Setup

### Prerequisites
* [Rust & Cargo](https://rustup.rs/) (Latest Stable)

### Installation
1. Clone the repository:
   ```bash
   git clone [https://github.com/rebjannik/MyShittyRougeLikeGame.git](https://github.com/rebjannik/MyShittyRougeLikeGame.git)
   cd MyShittyRougeLikeGame
