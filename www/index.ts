import init, { World, GameStateKind, DirectionKind } from "../pkg/snake_game.js";
import { rnd } from "./utils";
import paint from "./src/paint";
import { addKeyboardListener } from "./src/events";
import { canvas,ctx } from "./src/consts";

init().then((wasm) => {
	const GRID_WIDTH = 10;
	const RAND_START = rnd(GRID_WIDTH * GRID_WIDTH);
	const FPS = 5;
	const CELL_SIZE = 25;
	const SNAKE_START_SIZE = 3;


   	const world = World.new(GRID_WIDTH, RAND_START, SNAKE_START_SIZE);

    const worldWidth = world.width();

	canvas.height = worldWidth * CELL_SIZE + 1;
	canvas.width = worldWidth * CELL_SIZE + 1;

	

    
	const play = () => {
	
		const status = world.state();

		const paintProps = {
			ctx,
			world,
			wasm
		}
		 
		setTimeout(() => {
			ctx.clearRect(0, 0, canvas.width, canvas.height);
			paint(paintProps);
			world.step();
			requestAnimationFrame(play)
		}, 1000 / FPS);
	}

	addKeyboardListener(world);
	play();
});
