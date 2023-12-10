# Snake Game in Rust with WASM and TypeScript

Welcome to the Snake Game, an experimental project created by me. This implementation combines the power of Rust compiled to WebAssembly (WASM) and executed with TypeScript. The game is a basic rendition of the classic game Snake.

## Try the Game

You can try it out in your browser [here](https://willnjl.github.io/Snake-Game-Rust-Wasm/).

## Technologies Used

- **Rust**: The game logic and functionality are implemented in Rust, a powerful and performant systems programming language.

- **Cargo**: Rust's package manager and build system, used to manage dependencies and build the project.

- **WASM_Bindgen**: This tool facilitates communication between Rust and JavaScript, allowing the Rust code to be compiled to WebAssembly and integrated with the TypeScript code.

- **TypeScript**: The game is orchestrated and enhanced using TypeScript, a superset of JavaScript that adds static typing and other features.

- **Tailwind CSS**: A utility-first CSS framework used for styling the user interface, providing a clean and responsive design.

## Running the Project Locally

To run the project locally, follow these steps:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/willnjl/Snake-Game-Rust-Wasm.git
   ```

2. **Navigate to the Project Directory:**

   ```bash
   cd Snake-Game-Rust-Wasm
   ```

3. **Install Dependencies:**

   ```bash
   cargo watch -s "wasm-pack build --target web" -w "src" 
   ```

4. **Build and Serve the Project:**

   ```bash
   cd www/ && npm run dev
   ```

   This will build the Rust code to WebAssembly, compile the TypeScript code, and serve the project locally.

5. **Open in Browser:**

   Open your web browser and go to [http://localhost:8080](http://localhost:8080) to play the Snake Game.

## Feedback and Contributions

Your feedback is valuable! If you encounter issues, have suggestions, or want to contribute to the project, feel free to open an issue or submit a pull request.

Enjoy playing the Snake Game! 🐍🎮