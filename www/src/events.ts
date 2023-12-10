
import { DirectionKind, World, GameStateKind } from "snake_game";
import { gameControlBtn } from "./consts";

export const handleKeyPress = (e: KeyboardEvent, world: World) => {
	
	if (e.code === "ArrowLeft") {
		world.change_snake_direction(DirectionKind.Left)
	}
	if (e.code === "ArrowRight") {
		world.change_snake_direction(DirectionKind.Right)
	}
	if (e.code === "ArrowUp") {
		world.change_snake_direction(DirectionKind.Up)
	}
	if (e.code === "ArrowDown") {
		world.change_snake_direction(DirectionKind.Down)
	}


	world.step();
};


export const handleControlBtnClick = (event: MouseEvent, world: World) => {
	world.handle_click();
}