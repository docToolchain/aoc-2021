// Day 10: Syntax Scoring
// Solution by Christoph Jobmann

import { SyntaxChecker } from "./syntaxChecker";

let checker = new SyntaxChecker();
const scores = checker.getErrorScores();
console.log(scores.reduce((sum, v) => sum + v, 0));

console.log(checker.getMiddleCompletionScore());