import assert from "node:assert/strict";
import test from "node:test";
import { runSimulationChecks } from "./cases.js";

for (const result of runSimulationChecks()) {
  test(result.name, () => {
    assert.equal(result.passed, true, result.message);
  });
}
