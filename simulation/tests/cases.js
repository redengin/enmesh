import {
  CHANNEL_CAPACITY,
  createSimulation,
  getReachablePeers,
  stepSimulation,
} from "../src/simulation.js";

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function assertEveryNodeHasAReachablePeer(state) {
  for (const node of state.nodes) {
    assert(
      getReachablePeers(state, node).length > 0,
      `${node.id} has no peer within its transmit range at round ${state.tick}`,
    );
  }
}

export function runSimulationChecks() {
  const results = [];
  const test = (name, run) => {
    try {
      run();
      results.push({ name, passed: true });
    } catch (error) {
      results.push({
        name,
        passed: false,
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  test("every supported layout gives each node an in-range peer", () => {
    for (const nodeCount of [2, 3, 8, 9, 17, 32, 48]) {
      assertEveryNodeHasAReachablePeer(createSimulation({ nodeCount }));
    }
  });

  test("simulation rounds are repeatable for the same seed", () => {
    const first = createSimulation({ nodeCount: 24, traffic: 0.9, seed: 8721 });
    const second = createSimulation({ nodeCount: 24, traffic: 0.9, seed: 8721 });
    for (let round = 0; round < 12; round += 1) {
      stepSimulation(first);
      stepSimulation(second);
    }
    assert(
      JSON.stringify(first) === JSON.stringify(second),
      "Identical seeds produced different simulation state.",
    );
  });

  test("congestion advertises a shard and moves nodes onto it", () => {
    const state = createSimulation({ nodeCount: 8, traffic: 1, seed: 19 });
    for (let round = 0; round < 15; round += 1) stepSimulation(state);

    assert(state.shards.length > 0, "No shard was advertised under sustained load.");
    assert(
      state.nodes.some((node) => node.channelId !== "primary"),
      "No node joined the advertised shard.",
    );
    assert(
      state.lastStats.every(
        (stat) => stat.successes <= stat.attempts && stat.capacity === CHANNEL_CAPACITY,
      ),
      "Channel statistics exceeded their attempt count or used an unexpected capacity.",
    );
    assert(
      state.packetEvents.length > 0 &&
        state.packetEvents.every((packet) => typeof packet.channelId === "string"),
      "A broadcast was missing its channel identity.",
    );
  });

  test("range adaptation never removes a node's reachable peer", () => {
    const state = createSimulation({ nodeCount: 32, traffic: 1, seed: 91 });
    for (let round = 0; round < 24; round += 1) {
      stepSimulation(state);
      assertEveryNodeHasAReachablePeer(state);
    }
    assert(
      state.nodes.some((node) => node.range < 104),
      "No sharded node adjusted its transmit range.",
    );
  });

  return results;
}
