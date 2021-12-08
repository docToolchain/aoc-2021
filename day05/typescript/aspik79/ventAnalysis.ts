import * as r from './read';
import { VentLine } from './ventLine';

export class VentAnalysis {
    private lines: string[];

    constructor() {
        this.lines = r.readLines('input.txt', 'day05/typescript/aspik79');
    }

    /**
     * Counts the number of cells with heat vent counts greater than one.
     * @param includeDiagonal if false only horizontal and vertical vents count.
     * @returns count of cells with vent count greater than one.
     */
    public countHighVentAreas(includeDiagonal: boolean): number {
        const vents = this.lines.map(l => new VentLine(l, includeDiagonal));
        let heatMap = this.buildHeatMap(vents);

        return heatMap.flat().filter(n => n > 1).length;
    }

    private buildHeatMap(vents: VentLine[]) : number[][] {
        let heatMap: number[][] = [];

        for (const v of vents) {
            for (const c of v.listCovered()) {
                let row = heatMap[c[0]];
                if (row == undefined) {
                    // Make sure the row exists.
                    heatMap[c[0]] = [];
                }

                let n1 = heatMap[c[0]][c[1]];
                if (n1 == undefined) {
                    // Make sure the cell is initialized.
                    n1 = 0;
                }

                // Increment the heat map's cell value.
                heatMap[c[0]][c[1]] = n1 + 1;
            }
        }

        return heatMap;
    }
}