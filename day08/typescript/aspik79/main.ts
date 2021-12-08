// Day 8: Seven Segment Search
// Solution by Christoph Jobmann

import { DigitSearcher } from "./digitSearcher";

const ds = new DigitSearcher();
const simpleNumbers = ds.countSimpleDigits();
console.log(simpleNumbers.reduce((s, v) => s + v, 0));

const outputs = ds.collectOutputs();
console.log(outputs.reduce((s, v) => s + v, 0));
