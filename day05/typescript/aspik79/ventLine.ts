export class VentLine {
    private x1: number;
    private x2: number;
    private y1: number;
    private y2: number;

    constructor(line: string, private includeDiagonal: boolean) {
        const tokens = line.split(' ');
        const first = tokens[0].split(',');
        const second = tokens[2].split(',');
        this.x1 = parseInt(first[0]);
        this.y1 = parseInt(first[1]);
        this.x2 = parseInt(second[0]);
        this.y2 = parseInt(second[1]);
    }

    public isHorizontal(): boolean {
        return this.y1 == this.y2;
    }

    public isVertical(): boolean {
        return this.x1 == this.x2;
    }

    public listCovered(): number[][] {
        let result: number[][] = [];
        if (this.isHorizontal()) {
            for (let x = Math.min(this.x1, this.x2); x <= Math.max(this.x1, this.x2); ++x) {
                result.push([x, this.y1]);
            }
        }

        if (this.isVertical()) {
            for (let y = Math.min(this.y1, this.y2); y <= Math.max(this.y1,this.y2); ++y) {
                result.push([this.x1, y]);
            }
        }

        if (this.includeDiagonal && this.isDiagonal()) {
            if (this.x1 <= this.x2 && this.y1 <= this.y2) {
                for (let x = this.x1, y = this.y1; x <= this.x2, y <= this.y2; ++x, ++y) {
                    result.push([x, y]);
                }
            }

            if (this.x1 > this.x2 && this.y1 <= this.y2) {
                for (let x = this.x1, y = this.y1; x >= this.x2, y <= this.y2; --x, ++y) {
                    result.push([x, y]);
                }
            }

            if (this.x1 <= this.x2 && this.y1 > this.y2) {
                for (let x = this.x1, y = this.y1; x <= this.x2, y >= this.y2; ++x, --y) {
                    result.push([x, y]);
                }
            }

            if (this.x1 > this.x2 && this.y1 > this.y2) {
                for (let x = this.x1, y = this.y1; x >= this.x2, y >= this.y2; --x, --y) {
                    result.push([x, y]);
                }
            }
        }

        if (result.length == 0) {
            console.log('neither h, v nor d.')
        }

        return result;
    }

    public isDiagonal(): boolean {
        return Math.abs(this.x2 - this.x1) == Math.abs(this.y2 - this.y1);
    }
}