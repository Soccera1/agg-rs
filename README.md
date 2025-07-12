# 🎮 agg-rs

`agg-rs` is a fast-paced, terminal-based arithmetic game that challenges your math skills against the clock. Built with the performance and safety of Rust, it provides a simple yet engaging way to practice and test your calculation abilities.

## ✨ Features

- ⏱️ **Timed Challenges**: Set your own playtime and solve as many problems as you can before time runs out.
- 🎲 **Dynamic Problem Generation**: Faces a continuous stream of randomly generated addition, subtraction, multiplication, and division problems.
- 🔧 **Customizable Difficulty**: Adjust the maximum value of numbers used in problems to match your skill level.
- 📊 **Performance Tracking**: Get a final score and accuracy percentage to track your improvement.
- 📖 **In-Game Tutorial**: An optional tutorial to guide you through the gameplay.
- 👋 **Personalized Experience**: The game greets you by your username for a friendly touch.

## 🚀 How to Play

1.  **Start the Game**: Launch the application from your terminal.
2.  **View Tutorial (Optional)**: When prompted, you can choose to view a tutorial that explains the game mechanics.
3.  **Configure Your Game**:
    *   Choose whether to display your final accuracy as a whole number (integer) or a decimal (float).
    *   Set the total game duration in seconds.
    *   Define the maximum numerical value for the math problems.
4.  **Solve Problems**: Once the game starts, input your answers to the arithmetic problems as they appear.
5.  **See Your Results**: After the time expires, the game will display your final score and accuracy.

## 🛠️ Building from Source

### ✅ Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (includes `rustc` and `cargo`)
- A compatible C compiler (like GCC or Clang), which is required by a build dependency.

### 📦 Compilation

1.  Clone the repository:
    ```sh
    git clone https://github.com/soccera1/agg-rs.git
    cd agg-rs
    ```
2.  Build the project using Cargo:
    ```sh
    cargo build --release
    ```
3.  Run the executable:
    ```sh
    ./target/release/agg-rs
    ```
    Alternatively, you can compile and run the project in one step:
    ```sh
    cargo run
    ```

## 📄 License

This project is licensed under the terms of the LICENSE file.
