import { runSimulationChecks } from "./cases.js";

const resultsElement = document.querySelector("#test-results");
const summaryElement = document.querySelector("#test-summary");
const results = runSimulationChecks();

for (const result of results) {
  const item = document.createElement("li");
  item.className = result.passed ? "pass" : "fail";
  item.textContent = `${result.passed ? "PASS" : "FAIL"}  ${result.name}${
    result.message ? ` — ${result.message}` : ""
  }`;
  resultsElement.append(item);
}

const passedCount = results.filter((result) => result.passed).length;
const failedCount = results.length - passedCount;
summaryElement.textContent = `${passedCount} passed · ${failedCount} failed`;
summaryElement.classList.add(failedCount === 0 ? "pass" : "fail");
document.title = `${failedCount === 0 ? "PASS" : "FAIL"} — Simulation model tests`;
