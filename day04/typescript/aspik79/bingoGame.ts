import * as r from './read';

import { BingoCard } from "./bingoCard";

export class BingoGame {
    private blocks: string[];
    private draws: number[] = [];
    private cards: BingoCard[] = [];

    constructor() {
        this.blocks = r.readBlocks('input.txt', 'day04/typescript/aspik79', '\r\n\r\n');
    }

    /**
     * Simulate a bingo game with all cards and find the first winner.
     * @returns First winner card's score.
     */
    public selectFirstWinningCardScore() : number {
        this.initializeGame();
        for (let n of this.draws) {
            for (let c of this.cards) {
                if (c.markNumber(n)) {
                    // We have the first winner.
                    const cardScore = c.sumOfRemaining();
                    return n * cardScore;
                }
            }
        }

        return -1;
    }

    /**
     * Simulate a bingo game with all cards and find the last winner.
     * @returns Last winner card's score.
     */
     public selectLastWinningCardScore() : number {
        this.initializeGame();

        let lastScore = 0;

        // Keep track of cards that already won.        
        let winners: BingoCard[] = [];

        for (let n of this.draws) {
            for (let c of this.cards) {
                if (winners.findIndex(x => c == x) < 0 && c.markNumber(n)) {
                    // We have a new winner.
                    lastScore = c.sumOfRemaining() * n;
                    winners = winners.concat(c);
                }
            }
        }

        return lastScore;
    }

    private initializeGame() {
        this.draws = this.blocks[0].split(',').map(s => parseInt(s));

        const cardTexts = this.blocks.slice(1);
        this.cards = cardTexts.map(card => new BingoCard(card));
    }
}