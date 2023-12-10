import init, { World, GameStateKind, DirectionKind } from "snake_game";
import { rnd } from "./utils";
import paint from "./src/paint";
import { handleControlBtnClick, handleKeyPress } from "./src/events";
import { canvas,ctx, gameControlBtn } from "./src/consts";

init().then((wasm) => {
	const GRID_WIDTH = 10;
	const FPS = 10;
	const CELL_SIZE = 25;
	const SNAKE_START_LENGTH = 3;


   	const world = World.new(GRID_WIDTH, SNAKE_START_LENGTH);

    const worldWidth = world.width();

	canvas.height = worldWidth * CELL_SIZE + 1;
	canvas.width = worldWidth * CELL_SIZE + 1;

	

    
	const play = () => {
	
		const paintProps = {
			ctx,
			world,
			wasm
		}
		 
		setTimeout(() => {
			ctx.clearRect(0, 0, canvas.width, canvas.height);
			paint(paintProps);
			world.step();
			requestAnimationFrame(play);
		}, 1000 / FPS);
	}


	document.addEventListener('keydown', (e) => handleKeyPress(e, world));
	gameControlBtn.addEventListener('click', (e) => handleControlBtnClick(e, world));

	
	play();
});
