import init, { World, Direction, GameState } from "../pkg/snake_game.js";
import { rnd } from "./utils/rnd.js";

init().then((wasm) => {
	const GRID_WIDTH = 4;
	const RAND_START = rnd(GRID_WIDTH * GRID_WIDTH);
	const FPS = 5;
	const CELL_SIZE = 100;
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

    const paint = () => {
		drawWorld();
		// drawSnake();
		// drawReward();
		// drawGameStatus();
    }
    
    paint();
});
