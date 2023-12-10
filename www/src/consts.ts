import {rnd} from "../utils/index";

export const GRID_WIDTH = 20;
export const RAND_START = rnd(GRID_WIDTH * GRID_WIDTH);
export const FPS = 10;
export const CELL_SIZE = 10;
export const SNAKE_START_LENGTH = 3;
export const SNAKE_HEAD_COLOR = "#0D2818";
export const SNAKE_BODY_COLOR = "#058C42";


export const canvas = <HTMLCanvasElement>document.querySelector("#snake-canvas");
export const ctx = <CanvasRenderingContext2D>canvas.getContext("2d");
export const gameControlBtn = <HTMLButtonElement>document.getElementById('game-control-btn');
export const gameStatusText = <HTMLSpanElement>document.getElementById('game-status');
export const gamePointsText = <HTMLSpanElement>document.getElementById('game-points');
