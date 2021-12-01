// --- Day 1: Sonar Sweep ---
import * as fs from 'fs';

export class SonarSweeper {
    private lines: string[] = [];

    public RunSilver(){
        this.lines = this.Read();

        let previous: number = 0;
        let count: number = 0;

        for (let index = 0; index < this.lines.length; ++index) {
            const i = parseInt(this.lines[index]);
            if (previous != 0 && i > previous){
                ++count;
            }

            previous = i;
        }

        console.log(count);
    }

    public RunGold(){
        this.lines = this.Read();

        let sumPrevious: number = 0;
        let count: number = 0;

        for (let index = 0; index < this.lines.length - 2; ++index) {
            const i1 = parseInt(this.lines[index]);
            const i2 = parseInt(this.lines[index + 1]);
            const i3 = parseInt(this.lines[index + 2]);

            const sumCurrent = i1 + i2 + i3;
            if (sumPrevious != 0 && sumPrevious < sumCurrent){
                ++count;
            }

            sumPrevious = sumCurrent;
        }

        console.log(count);
    }

    
    Read() : string[] {
        let input: string;

        if (fs.existsSync('day01/typescript/aspik79/input.txt')) {
            input = fs.readFileSync('day01/typescript/aspik79/input.txt','utf8');
        }
        else {
            input = fs.readFileSync('input.txt','utf8');
        }

        return input.split('\r\n')
    }    
}
