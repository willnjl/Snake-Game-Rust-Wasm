import init, { World, GameStateKind } from "../pkg/snake_game.js";
import { rnd } from "./utils/rnd.js";

init().then((wasm) => {
	const GRID_WIDTH = 10;
	const RAND_START = rnd(GRID_WIDTH * GRID_WIDTH);
	const FPS = 5;
	const CELL_SIZE = 25;
	const SNAKE_START_SIZE = 3;


   	const world = World.new(GRID_WIDTH, RAND_START, SNAKE_START_SIZE);

    const worldWidth = world.width();

	const canvas = <HTMLCanvasElement>document.querySelector("#snake-canvas");
	const ctx = <CanvasRenderingContext2D>canvas.getContext("2d");

	canvas.height = worldWidth * CELL_SIZE + 1;
	canvas.width = worldWidth * CELL_SIZE + 1;


	function drawWorld() {
		ctx.strokeStyle = "#ccc";
		ctx.beginPath();


		for (let x = 0; x < worldWidth + 1; x++) {
			ctx.moveTo(CELL_SIZE * x + 1, 0);
			ctx.lineTo(CELL_SIZE * x + 1, worldWidth * CELL_SIZE);
		}

		for (let y = 0; y < worldWidth + 1; y++) {
			ctx.moveTo(0, CELL_SIZE * y + 1);
			ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y + 1);
		}

		ctx.stroke();
	}

	function drawSnake() {

		
		
		let snakeCells = new Uint32Array(
			wasm.memory.buffer,
			world.snake_cells(),
			world.snake_length(),
		);
		
			
			snakeCells.slice().reverse().forEach((cell, i) => {
			const index = cell;
			const posX = index % worldWidth;
			const posY = Math.floor(index / worldWidth);
			ctx.fillStyle = i === snakeCells.length - 1 ? "#FF0099" : "#AA0099"
			ctx.beginPath();
			
			ctx.moveTo(posX, posY);
			ctx.fillRect(posX * CELL_SIZE, posY * CELL_SIZE, CELL_SIZE, CELL_SIZE);
			ctx.stroke();
		})


	}


	function drawReward() {
		const cellIndex = world.reward_cell();
		const posX = cellIndex % worldWidth;
		const posY = Math.floor(cellIndex / worldWidth);
		
		ctx.fillStyle = "#FF0000"
		ctx.beginPath();
		ctx.fillRect(
			posX * CELL_SIZE,
			posY * CELL_SIZE,
			CELL_SIZE,
			CELL_SIZE
		)
		ctx.stroke();



		if (cellIndex == undefined) {
			alert("Your Won!");
		}
	}
    const paint = () => {
		drawWorld();
		drawSnake();
		drawReward();
		// drawGameStatus();
    }
    
	const play = () => {
	
		const status = world.state();
		
		setTimeout(() => {
			ctx.clearRect(0, 0, canvas.width, canvas.height);
			paint();
			world.step();
			requestAnimationFrame(play)
		}, 1000 / FPS);
	}

	play();
});
