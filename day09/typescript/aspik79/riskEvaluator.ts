import * as r from './read';

export class RiskEvaluator {
    private heightMap: number[][] = [];

    constructor() {
        const lines = r.readLines('input.txt', '');
        this.heightMap = lines.map(l => l.split('').map(s => parseInt(s)));
    }

    /**
     * Recovers the values of high risk.
     * @returns High risk values.
     */
    public getHighRiskValues() : number[] {
        const locs = this.getHighRiskLocations();

        return locs.map(l => 1 + this.heightMap[l[0]][l[1]]);
    }

    /**
     * Gets the sizes of the individual basins, extending from a low point.
     * @returns Sizes of basins.
     */
    public getBasins() : number[] {
        let result: number[] = [];
        const lowPoints = this.getHighRiskLocations();

        for (let origin of lowPoints) {

            let seen: number[][] = [];
            let basin = [origin];
            let unexplored = basin.slice(0);

            do {
                unexplored = this.tryExtendBasin(basin, unexplored, seen);
            }
            while (unexplored.length > 0);

            result.push(basin.length);
        }

        return result;
    }

    private getHighRiskLocations(): number[][] {
        let result: number[][] = [];
        for (let row = 0; row < this.heightMap.length; ++row) {
            const rowData = this.heightMap[row];
            for (let col = 0; col < rowData.length; ++col) {
                const v = rowData[col];
                if (this.calcRisk(v, row - 1, col)
                    && this.calcRisk(v, row, col - 1)
                    && this.calcRisk(v, row + 1, col)
                    && this.calcRisk(v, row, col + 1)) {
                    result.push([row, col]);
                }
            }
        }

        return result;

    }

    private tryExtendBasin(basin: number[][], unexplored: number[][], seen: number[][]) : number[][] {
        let nextUnexplored: number[][] = [];

        for (let point of unexplored) {
            if (!seen.some(b => b[0] == point[0] && b[1] == point[1])) {
                const v = this.heightMap[point[0]][point[1]];
                this.tryExtendBasinStep(basin, nextUnexplored, v, point[0] - 1, point[1]);
                this.tryExtendBasinStep(basin, nextUnexplored, v, point[0] + 1, point[1]);
                this.tryExtendBasinStep(basin, nextUnexplored, v, point[0], point[1] - 1);
                this.tryExtendBasinStep(basin, nextUnexplored, v, point[0], point[1] + 1);
                seen.push(point);
            }
        }

        return nextUnexplored;
    }

    private tryExtendBasinStep(basin: number[][], nextUnexplored: number[][], v: number, r: number, c: number) {

        if (r >= 0 && r <= this.heightMap.length - 1 && c >= 0 && c <= this.heightMap[r].length - 1) {
            if (this.heightMap[r][c] < 9 && v <= this.heightMap[r][c] && !basin.includes([r,c])) {
               
                if (basin.some(b => b[0] == r && b[1] == c)) {
                }
                else {
                    basin.push([r, c]);
                }

                nextUnexplored.push([r, c]);
            }
        }
    }

    private calcRisk(v: number, r: number, c: number): boolean {
        if (r >= 0 && r <= this.heightMap.length - 1 && c >= 0 && c <= this.heightMap[r].length - 1) {
            return v < this.heightMap[r][c];
        }

        return true;
    }
}