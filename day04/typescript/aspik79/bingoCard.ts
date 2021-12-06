export class BingoCard {
    private numbers: number[];
    private columns: number;
    private rows: number;
    private columnsMatched: number[];
    private rowsMatched: number[];

    /**
     * Creates a new bingo card.
     * @param block Block of numbers describing the bingo card's content.
     */
    constructor(private block: string) {
        const lines = block.split('\r\n');

        // Parse the numbers. Careful with duplicate space separation.
        const flat = lines.flatMap(l => l.split(' '));
        this.numbers = flat.map(s => parseInt(s)).filter(n => n >= 0);

        this.rows = lines.length;
        this.columns = this.numbers.length / this.rows;
        
        // Initialize match counts; no matches yet.
        this.rowsMatched = Array<number>(this.rows).fill(0);
        this.columnsMatched = Array<number>(this.columns).fill(0);
    }

    /**
     * Marks the given number on the bingo card if it exists. Also contains a win-check.
     * @param n number that is to be marked.
     * @returns true if there is a bingo match, otherwise false.
     */
    public markNumber(n: number) : boolean {
        let result = false;
        
        const index = this.numbers.indexOf(n);
        if (index >= 0) {
            // There is a match! Calculate row and column.
            const col = index % this.columns;
            const row = Math.floor(index / this.columns);

            // Take the number off the list.
            this.numbers[index] = NaN;

            // Update match counts and check for win.
            result ||= ++this.columnsMatched[row] == this.columns;
            result ||= ++this.rowsMatched[col] == this.rows;
        }

        return result;
    }

    /**
     * Calculates the sum of the remaining numbers.
     * @returns Sum of remaining numbers.
     */
    public sumOfRemaining() : number {
        return this.numbers.filter(n => n).reduce((sum, current) => sum + current, 0);
    }
}