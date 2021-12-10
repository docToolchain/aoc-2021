import * as r from './read';

export class SyntaxChecker {
    private tokens: string[][] = [
        ['(',')'],
        ['[',']'],
        ['{','}'],
        ['<','>']
    ];

    private score: Map<string, number>;

    private completionScoreOrder = this.tokens.map(t => t[1]);

    private lines: string[];

    constructor() {
        this.score = new Map<string, number>();
        this.score.set(')', 3);
        this.score.set(']', 57);
        this.score.set('}', 1197);
        this.score.set('>', 25137);

        this.lines = r.readLines('input.txt', '');
    }

    /**
     * Gets the error scores for the given lines.
     * @returns Error scores.
     */
    public getErrorScores() : number[] {
        let result: number[] = [];

        result = this.lines.map(l => this.getErrorScore(l));

        return result;
    }

    /**
     * Gets the median completion score.
     * @returns median completion score.
     */
    public getMiddleCompletionScore() : number {
        let cscores = this.getCompletionScores();
        cscores.sort((a, b) => a - b);

        const middleIndex = (cscores.length - 1) / 2;
        return cscores[middleIndex];
    }

    private getCompletionScores() : number[] {
        return this.filterToIncomplete().map(l => this.getCompletionScore(l));
    }

    private getCompletionScore(line: string) : number {
        let remain: string[] = [];

        this.process(line.split(''), remain);

        remain.reverse();
        let completingTokens = remain.map(i => this.findCompleting(i));
        return completingTokens.reduce((p, v) => p * 5 + (this.completionScoreOrder.indexOf(v) + 1), 0);
    }

    private findCompleting(s: string) : string {
        return this.tokens.filter(t => t[0] == s).map(t => t[1])[0];
    }

    private filterToIncomplete() : string[] {
        return this.lines.filter(l => this.getErrorScore(l) == 0);
    }

    private process(remain: string[], started: string[]) : string {

        while (remain.length > 0) {
            let char = remain.shift() as string;
            if (this.tokens.some(t => t[0] == char)) {
                started.push(char);
            }
            else {
                let last = started.pop() as string;
                const expectedCompleting = this.findCompleting(last);
                if (expectedCompleting != char) {
                    return char;
                }
            }
        }

        return '';
    }

    private getErrorScore(line: string) : number {
        let errorChar = this.process(line.split(''), []);
        let result = this.score.get(errorChar);
        return result || 0;
    }
}