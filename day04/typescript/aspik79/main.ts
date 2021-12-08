// Day 04: Giant Squid by Christoph Jobmann

import { BingoGame } from "./bingoGame";

const game = new BingoGame();
let score = game.selectFirstWinningCardScore();
console.log(score);

score = game.selectLastWinningCardScore();
console.log(score);