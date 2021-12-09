import * as r from './read';

/**
 * Class for evalualting fuel for minimum crab-in-a-sub alignment.
 */
export class CrabAligner {
    private positions: number[];
    
    /**
     * Initializes a new instance of the CrabAligner class.
     * @constantFuel If true fuel consumption is constant, otherwise linear.
     */
    constructor(private constantFuel: boolean) {
        const line = r.readLines('input.txt', '')[0];
        this.positions = line.split(',').map(s => parseInt(s));
    }

    /**
     * Count the minimum amount of fuel consumption for alignment.
     */
    public countMinAlignmentFuel() {
        const min = Math.min(...this.positions);
        const max = Math.max(...this.positions);

        let minFuel = this.countAlignment(min);

        for (let target = min + 1; target <= max; ++target) {
            const fuel = this.countAlignment(target);
            if (fuel < minFuel) {
                minFuel = fuel;
            }
        }

        return minFuel;
    }

    private countAlignment(target: number) : number {
        return this.positions.reduce((sum, current) => sum + this.calculateFuel(Math.abs(current - target)), 0);
    }

    private calculateFuel(distance: number) : number {
        // Thanks to Mr. Gauss for simplified calcualtion of aggregated fuel use.
        return this.constantFuel ? distance : distance * (distance + 1) / 2;
    }
}