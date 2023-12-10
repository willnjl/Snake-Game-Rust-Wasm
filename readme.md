# Snake Game in Rust with WASM and TypeScript

 The game is a basic rendition of the classic game Snake. This project was a just for fun example to get familiar with the Rust programming language. This implementation combines the power of Rust compiled to WebAssembly (WASM) and executed with TypeScript

It is based of the the Rust Udemy Course from [Jerga99](https://github.com/Jerga99)

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

3. **Compile Rust:**

   ```bash
   wasm-pack build --target web
   ```

4. **Generate the Frontend**

   ```bash
   cd www/ && npm run dev
   ```

   This will build the Rust code to WebAssembly, compile the TypeScript code, and serve the project locally.

5. **Open in Browser:**

   Open your web browser and go to [http://localhost:8080](http://localhost:8080) to play the Snake Game.

