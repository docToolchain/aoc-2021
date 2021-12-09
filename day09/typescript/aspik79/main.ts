// Day 9: Smoke Basin
// Solution by Christoph Jobmann

import { RiskEvaluator } from "./riskEvaluator";

const re = new RiskEvaluator();

const highRisk = re.getHighRiskValues();
console.log(highRisk.reduce((s, v) => s + v, 0));

const basins = re.getBasins();
basins.sort((a, b) => b - a);
console.log(basins.slice(0, 3).reduce((p, v) => p * v, 1));