import * as r from './read';

export class BinaryDiagnostic {
    private lines: string[];

    constructor() {
        this.lines = r.readLines('input.txt', 'day03/typescript/aspik79');
    }

    /**
     * Part 1: evaluate the power consumption.
     * @returns the power consumption.
     */
    public EvaluatePowerConsumption() : number {
        let gamma = 0;
        let epsilon = 0;

        const bitCount = this.lines[0].length;

        for (let index = 0; index < bitCount; ++index) {

            const bits = this.lines.map(l => l[index]);

            // Count ones and zeros at current index.
            const zeros = bits.filter(b => b == '0').length;
            const ones = bits.filter(b => b == '1').length;

            const mostCommon = this.mostCommonBit(bits);

            // shift the decimal numbers before inspecting the next bit.
            gamma <<= 1;
            epsilon <<= 1;

            if (mostCommon == '1') {
                gamma += 1;
            }
            else if (mostCommon == '0'){
                epsilon += 1;
            }
            else {
                console.log(`equal count of ones and zeroes at index ${index} unexpected.`);
            }
        }

        return gamma * epsilon;
    }

    /**
     * Part 2: evaluate the life support value.
     * @returns the life support value.
     */
     public EvaluateLifeSupport() : number {
        let o2 = this.EvaluateOxygenGenerator(this.lines, 0);
        let co2 = this.EvaluateCo2Scrubber(this.lines, 0);

        console.log(o2);
        console.log(co2);

        const o2value = binaryToInt(o2.join(''));
        const co2value = binaryToInt(co2.join(''));

        console.log(o2value);
        console.log(co2value);
      
        return o2value * co2value;
    }

    /**
     * Recursively reduce the input according to additional input extracted based on the given index.
     * @param lines input lines.
     * @param index index that defines the current filter.
     * @returns filtered lines.
     */
    private EvaluateOxygenGenerator(lines: string[], index: number) : string[] {
        const bit = this.mostCommonBit(lines.map(l => l[index]));

        let result = lines.filter(l => l[index] == bit);
        if (result.length > 1) {
            //console.log('o2: ' + result.length + ' lines remaining.')
            result = this.EvaluateOxygenGenerator(result, ++index);
        }

        return result;
    }

    /**
     * Recursively reduce the input according to additional input extracted based on the given index.
     * @param lines input lines.
     * @param index index that defines the current filter.
     * @returns filtered lines.
     */
     private EvaluateCo2Scrubber(lines: string[], index: number) : string[] {
        const bit = this.LeastCommonBit(lines.map(l => l[index]));

        let result = lines.filter(l => l[index] == bit);
        if (result.length > 1) {
            //console.log('co2: ' + result.length + ' lines remaining.')
            result = this.EvaluateCo2Scrubber(result, ++index);
        }

        return result;
    }

    private LeastCommonBit(bits: string[]) : string {
        const zeros = bits.filter(b => b == '0').length;
        const ones = bits.filter(b => b == '1').length;

        return ones >= zeros ? '0' : '1';
    }

    private mostCommonBit(bits: string[]) : string {
        const zeros = bits.filter(b => b == '0').length;
        const ones = bits.filter(b => b == '1').length;

        return ones >= zeros ? '1' : '0';
    }
}

function binaryToInt(bits: string) : number {
    let result = 0;
    for (const b of bits) {
        result <<= 1;
        result += b == '1' ? 1 : 0;
    }

    return result;
}