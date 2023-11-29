
import { DirectionKind, World } from "snake_game"

export const addKeyboardListener = (world: World) => {
    document.addEventListener('keydown', ({code}) => {
		if (code === "ArrowLeft") {
			world.change_snake_direction(DirectionKind.Left)
		}
		if (code === "ArrowRight") {
			world.change_snake_direction(DirectionKind.Right)
		}
		if (code === "ArrowUp") {
			world.change_snake_direction(DirectionKind.Up)
		}
		if (code === "ArrowDown") {
			world.change_snake_direction(DirectionKind.Down)
		}
	})
}