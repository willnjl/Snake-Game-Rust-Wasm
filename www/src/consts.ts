import {rnd} from "../utils/index";

export const GRID_WIDTH = 10;
export const RAND_START = rnd(GRID_WIDTH * GRID_WIDTH);
export const FPS = 5;
export const CELL_SIZE = 25;
export const SNAKE_START_SIZE = 3;
export const canvas = <HTMLCanvasElement>document.querySelector("#snake-canvas");
export const ctx = <CanvasRenderingContext2D>canvas.getContext("2d");
export const gameControlBtn = <HTMLButtonElement>document.getElementById('game-control-btn');
export const gameStatusText = <HTMLSpanElement>document.getElementById('game-status');
export const gamePointsText = <HTMLSpanElement>document.getElementById('game-points');
