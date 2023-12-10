import { CELL_SIZE, SNAKE_BODY_COLOR, SNAKE_HEAD_COLOR, gameControlBtn } from "./consts"; 
import { World } from "snake_game";
import { InitOutput } from "snake_game";
import { gameStatusText, gamePointsText } from "./consts";


type DrawProps = { 
    world: World,
    ctx: CanvasRenderingContext2D,
    wasm: InitOutput
}

function drawWorld({world, ctx}: DrawProps) {
    const worldWidth = world.width();
    ctx.strokeStyle = "#eee";
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

function drawSnake({wasm, world, ctx}: DrawProps) {
    const worldWidth = world.width();
    
    let snakeCells = new Uint32Array(
        wasm.memory.buffer,
        world.snake_cells(),
        world.snake_length(),
    );
    
        
        snakeCells.slice().reverse().forEach((cell, i) => {
        const index = cell;
        const posX = index % worldWidth;
        const posY = Math.floor(index / worldWidth);
            ctx.fillStyle = i === snakeCells.length - 1 ? SNAKE_HEAD_COLOR : SNAKE_BODY_COLOR;
        ctx.beginPath();
        
        ctx.moveTo(posX, posY);
        ctx.fillRect(posX * CELL_SIZE, posY * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
    })


}


function drawReward({ world, ctx }: DrawProps) {
    const worldWidth = world.width();
    
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

function drawGameStatus({world}: DrawProps) {

    gameStatusText.innerHTML = world.game_status_text();
    gamePointsText.innerHTML = world.points().toString();

    gameControlBtn.innerHTML = world.game_btn_text();

}


const paint = (props:DrawProps) => {
    drawWorld(props);
    drawSnake(props);
    drawReward(props);
    drawGameStatus(props);
}


export default paint;
