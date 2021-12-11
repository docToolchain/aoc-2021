// Day 11: Dumbo Octopus
// Solution by Christoph Jobmann 

import { FlashAnalyzer } from "./flashAnalyzer";

const flashes = new FlashAnalyzer().countFlashesAfter(100);
console.log(flashes);

const untilFullFlash = new FlashAnalyzer().countStepsToFullFlash();
console.log(untilFullFlash);
