import * as r from './read';

export class FishPopulation {
    private line: string;
    private startPopulation: number[];
    private interval = 7;
    private offset = 2;

    constructor() {
        this.line = r.readLines('input.txt', '')[0];
        this.startPopulation = this.line.split(',').map(s => parseInt(s));
    }

    /**
     * Straightforward approach for simulating popolation. Sufficient for first star.
     * @param steps Number of steps that are to be simulated.
     * @returns population size after the given number of steps.
     */
    public getPopulationAfter(steps: number) : number {
        let pop = this.startPopulation.slice(0);
        for (let c = 0; c < steps; ++c){
            pop = this.advancePopulation(pop);
        }

        return pop.length;
    }

    /**
     * Smarter approach that is more efficient.
     * @param steps Number of steps that are to be simulated.
     * @returns population size after the given number of steps.
     */
    public getPopulationSmartAfter(steps: number) : number {
        let pop = new Array<number>(this.interval).fill(0);

        for (let timer of this.startPopulation) {
            ++pop[timer];
        }

        for (let c = 0; c < steps; ++c){
            pop = this.advancePopulationSmart(pop);
        }

        return pop.reduce((sum, p) => sum + p, 0);
    }

    private advancePopulation(current: number[]) : number[] {
        const next = current.slice(0);
        
        for (const index in current) {
            const timer = current[index] - 1;
            if (timer < 0) {
                // Timer goes below 0 - time for reproduction.
                // Add the offspring.
                next.push(this.interval - 1 + this.offset);

                // Reset timer.
                next[index] = this.interval - 1
            }
            else {
                // Apply reduced timer.
                next[index] = timer;
            }
        }

        return next;
    }

    
    private advancePopulationSmart(current: number[]) : number[] {
        const next: number[] = Array<number>(this.interval + this.offset).fill(0);
        
        for (const index in current) {

            let x: number = parseInt(index);
            const timer = parseInt(index) - 1;
            if (timer < 0) {
                // Timer goes below 0 - time for reproduction.
                // Add the offspring.
                next[this.interval - 1 + this.offset] = (current[index] ?? 0);

                // Reset timer.
                next[this.interval - 1] = current[index];
            }
            else {
                // Shift amount of fishes for timer value.
                next[timer] += (current[index] ?? 0);
            }
        }

        return next;
    }
}
