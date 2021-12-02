// --- Day 2: Dive! ---
import * as fs from 'fs';

export class Dive {

    private lines: string[];
    private position = 0;
    private depth = 0;
    private aim = 0;

    constructor() {
        this.lines = this.Read();
    }

    /** Does the dive for the first part. */
    public RunSilver() : void {
        this.DoDive((c, n) => this.silverNavigation(c, n));
    }

    /** Does the dive for the second part. */
    public RunGold() : void {
        this.DoDive((c, n) => this.goldNavigation(c, n));
    }

    private DoDive(step: (c: string, a: number) => void) : void{
        this.position = 0;
        this.depth = 0;
        this.aim = 0;

        for (const line of this.lines) {
            // Recover command and amount an apply them to the step function.
            const tokens: string[] = line.split(' ');
            const amount = parseInt(tokens[1]);
            step(tokens[0], amount);
        }

        console.log('position: ' + this.position);
        console.log('depth: ' + this.depth);
        console.log('product: ' + this.position * this.depth);
    }

    private silverNavigation (command: string, amount: number) {
        switch (command) {
            case 'up':
                this.depth -= amount;
                break;
            case 'down':
                this.depth += amount;
                break;
            case 'forward':
                this.position += amount;
                break;
            default:
                console.warn(`token ${command} unexpected.`);
        };
    }

    private goldNavigation (command: string, amount: number) {
        switch (command) {
            case 'up':
                this.aim -= amount;
                break;
            case 'down':
                this.aim += amount;
                break;
            case 'forward':
                this.position += amount;
                this.depth += amount * this.aim;
                break;
            default:
                console.warn(`token ${command} unexpected.`);
        };
    }

    private Read() : string[] {
        let input: string;

        const prefix = 'day02/typescript/aspik79/';
        const path1 = prefix + 'input.txt';
        
        if (fs.existsSync(path1)) {
            input = fs.readFileSync(path1,'utf8');
        }
        else {
            input = fs.readFileSync('input.txt','utf8');
        }

        return input.split('\r\n')
    }
}