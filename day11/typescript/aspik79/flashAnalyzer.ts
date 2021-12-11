import * as r from './read';

export class FlashAnalyzer {
    private energyLevels: number[][] = [];
    private columns: number;
    private rows: number;

    constructor() {
        const lines = r.readLines('input.txt', '');
        this.rows = lines.length;
        this.columns = lines[0].length;

        for (let r = 0; r < this.rows; ++r) {
            const row = lines[r];
            this.energyLevels[r] = [];
            for (let c = 0; c < this.columns; ++c) {
                this.energyLevels[r][c] = parseInt(row.charAt(c));
            }
        }
    }

    /**
     * Counts home many flashes occurred within the given number of steps.
     * @param steps number of desired steps.
     * @returns number of flashes.
     */
    public countFlashesAfter(steps: number) : number {
        let result = 0;

        for (let c = 0; c < steps; ++c) {
            result += this.step();
        }

        return result;
    }

    /**
     * Counts how many steps it takes until all octopuses flash at the same time.
     * @returns Number of steps.
     */
    public countStepsToFullFlash() : number {
        for (let result = 0; true; ++result) {
            if (this.step() == this.rows * this.columns) {
                return result;
            }
        }
    }

    private step() : number {
        let result = 0;
        let flashed: boolean[] = new Array<boolean>(this.energyLevels.length * this.energyLevels[0].length).fill(false);

        for (let r = 0; r < this.rows; ++r) {
            for (let c = 0; c < this.columns; ++c) {
                result += this.incrementAndFlash(r, c, flashed);
            }
        }

        // Reset energy for flashers.
        for (let r = 0; r < this.rows; ++r) {
            for (let c = 0; c < this.columns; ++c) {
                if (this.energyLevels[r][c] > 9) {
                    this.energyLevels[r][c] = 0;
                }
            }
        }

        return result;
    }

    private incrementAndFlash(r: number, c: number, flashed: boolean[]) : number {
        let result = 0;
        if (r >= 0 && r < this.rows && c >= 0 && c < this.columns) {
            if (++this.energyLevels[r][c] > 9 && !flashed[r * this.rows + c]) {
                const neighbours = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
                flashed[r * this.rows + c] = true;
                result = 1 + neighbours
                    .map(p => this.incrementAndFlash(r + p[0], c + p[1], flashed))
                    .reduce((s, v) => s + v, 0);
            }
        }

        return result;
    }
}