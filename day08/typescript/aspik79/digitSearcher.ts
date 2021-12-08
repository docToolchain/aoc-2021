import * as r from './read';

export class DigitSearcher {
    private lines: string[];

    private patterns: string[] = [
        'abcefg',
        'cf',
        'acdeg',
        'acdfg',
        'bcdf',
        'abdfg',
        'abdefg',
        'acf',
        'abcdefg',
        'abcdfg'
    ];
    
    private segmentCountForNumber = this.patterns.map(p => p.length);

    private simpleNumbers = [1, 4, 7, 8];

    /**
     * Initializes a new instance of the DigitSearcher class.
     */
    constructor() {
        this.lines = r.readLines('input.txt', '');
    }

    /**
     * Counts the occurrence of easy-to-decode digits.
     * @returns Amount of decoded digits (if they were easy to decode).
     */
    public countSimpleDigits() : number[] {
        let result: number[] = new Array<number>(10).fill(0);

        for (let line of this.lines) {
            let [signal, output] = line.split(' | ');

            const tokens = output.split(' ');
            for (let token of tokens) {
                for (let n of this.simpleNumbers) {
                    if (token.length == this.segmentCountForNumber[n]) {
                        ++result[n];
                    }
                }
            }
        }

        return result;
    }

    /**
     * Collect all translated outputs.
     * @returns Translated outputs.
     */
    public collectOutputs() : number[] {
        let result: number[] = [];

        for (let line of this.lines) {
            result.push(this.processLine(line));
        }

        return result;
    }

    private processLine(line: string) : number {
        const [signal, output] = line.split(' | ');

        const pattern: string[] =  this.determinePatterns(signal);

        let result = 0;

        let otokens = sortedTokens(output.split(' '));
        for (let otoken of otokens) {
            const idx = pattern.indexOf(otoken);
            if (idx >= 0) {
                result = result * 10 + idx;
            }
        }

        return result;
    }

    private determinePatterns(signal: string) : string[] {
        let tokens: string[] = sortedTokens(signal.split(' '));
    
        let pattern: string[] = Array<string>(10).fill('0');

        // Pattern extraction was solved by human - not sure how to do this via code without trying all permutations.
        pattern[1] = this.tokensCandidatesForNumber(tokens, 1)[0];
        pattern[4] = this.tokensCandidatesForNumber(tokens, 4)[0];
        pattern[7] = this.tokensCandidatesForNumber(tokens, 7)[0];
        pattern[8] = this.tokensCandidatesForNumber(tokens, 8)[0];

        pattern[6] = tokens.filter(t => t.length == this.segmentCountForNumber[6]
            && except(pattern[1], t).length == 1)[0];

        pattern[9] = tokens.filter(t => t.length == this.segmentCountForNumber[9]
            && intersect(pattern[4], t).length == 4)[0];

        pattern[0] = tokens.filter(t => t.length == this.segmentCountForNumber[0]
            && t != pattern[6] && t != pattern[9])[0];

        pattern[3] = this.tokensCandidatesForNumber(tokens, 3)
            .filter(t => intersect(t, pattern[1]).length == 2)[0];

        pattern[5] = this.tokensCandidatesForNumber(tokens, 5)
            .filter(t => intersect(t, pattern[6]).length == 5)[0];

        pattern[2] = this.tokensCandidatesForNumber(tokens, 2)
            .filter(t => !pattern.some(p => p == t))[0];

        return pattern;
    }

    private tokensCandidatesForNumber(tokens: string[], n: number) : string[] {
        return tokens.filter(t => t.length == this.segmentCountForNumber[n]);
    }
}

/**
 * Creates an array of sorted single characters from the string.
 */
function toSortedCharArray(s: string) : string[] {
    let result: string[] = [];
    for (let i = 0; i < s.length; ++i) {
        result.push(s.charAt(i));
    }

    result.sort((s1, s2) => s1.localeCompare(s2));

    return result;
}

/**
 * Processes the given tokens, creating a sorted representation of each token.
 * The order of tokens is preserved.
 */
function sortedTokens(tokens: string[]) : string[] {
    return tokens.map(t => toSortedCharArray(t).join(''));
}

/**
 * Determines the characters that are part of both strings.
 * @param s1 First string.
 * @param s2 Second string.
 * @returns Array of single characters that are part of both string.
 */
function intersect(s1: string, s2: string) : string[] {
    const expanded = toSortedCharArray(s2);
    let result: string[] = [];

    for (let x1 of toSortedCharArray(s1)) {
        if (expanded.includes(x1)) {
            result.push(x1);
        }
    }

    return result;
}

/**
 * Determines the characters that are part of the first but not the second string.
 * @param s1 First string.
 * @param s2 Second string.
 * @returns Array of single characters that are part of the first but not the second string.
 */
 function except(s1: string, s2: string) : string[] {
    const expanded = toSortedCharArray(s2);
    let result: string[] = [];
    for (let x1 of toSortedCharArray(s1)) {
        if (!s2.includes(x1)) {
            result.push(x1);
        }
    }

    return result;
}
