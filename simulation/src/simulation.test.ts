import { describe, expect, it } from "vitest";
import { floodingStrategy } from "./flood";
import {
  runSimulation,
  type MeshNode,
  type PropagationStrategy,
} from "./simulation";

const line: MeshNode[] = [
  { id: "a", position: { x: 0, y: 0 } },
  { id: "b", position: { x: 10, y: 0 } },
  { id: "c", position: { x: 20, y: 0 } },
];

function run(
  nodes: readonly MeshNode[],
  strategy: PropagationStrategy = floodingStrategy(0),
  seed = 7,
) {
  return runSimulation(
    { nodes, sourceId: "a", range: 10, seed },
    strategy,
  );
}

describe("runSimulation", () => {
  it("propagates over range-limited links and terminates when all nodes have forwarded", () => {
    const result = run(line);

    expect([...result.reachedNodeIds].sort()).toEqual(["b", "c"]);
    expect(result.maxHop).toBe(2);
    expect(result.transmissionCount).toBe(3);
    expect(result.collisionCount).toBe(0);
    expect(result.frames[0]?.slot).toBe(0);
  });

  it("does not deliver to nodes outside radio range", () => {
    const result = run([
      line[0]!,
      { id: "far", position: { x: 25, y: 0 } },
    ]);

    expect(result.reachedNodeIds.size).toBe(0);
    expect(result.transmissionCount).toBe(1);
  });

  it("counts simultaneous arrivals as a receiver collision", () => {
    const nodes: MeshNode[] = [
      { id: "a", position: { x: 0, y: 0 } },
      { id: "b", position: { x: 10, y: 0 } },
      { id: "c", position: { x: 0, y: 10 } },
      { id: "d", position: { x: 10, y: 10 } },
    ];
    const strategy: PropagationStrategy = {
      name: "simultaneous",
      createRun: () => ({
        onStart: () => [0],
        onReception: () => [1],
      }),
    };

    const result = runSimulation(
      {
        nodes,
        sourceId: "a",
        range: 10,
        seed: 1,
      },
      strategy,
    );

    expect(result.reachedNodeIds).toEqual(new Set(["b", "c"]));
    expect(result.collisionCount).toBe(2);
    expect(result.successfulReceptionCount).toBe(2);
    expect(result.frames[1]?.collidedReceivers).toEqual(["a", "d"]);
  });

  it("produces identical frames and metrics for the same seed", () => {
    const nodes: MeshNode[] = [
      line[0]!,
      line[1]!,
      { id: "d", position: { x: 10, y: 8 } },
    ];
    const first = run(nodes, floodingStrategy(5), 123);
    const second = run(nodes, floodingStrategy(5), 123);

    expect(first.frames).toEqual(second.frames);
    expect(first.transmissionCount).toBe(second.transmissionCount);
    expect(first.collisionCount).toBe(second.collisionCount);
  });

  it("lets a custom strategy choose whether a reception is forwarded", () => {
    const strategy: PropagationStrategy = {
      name: "single hop",
      createRun: () => ({
        onStart: () => [0],
        onReception: (reception) =>
          reception.receiverId === "b" ? [3] : [],
      }),
    };

    const result = run(line, strategy);
    expect(result.reachedNodeIds).toEqual(new Set(["b", "c"]));
    expect(result.frames.map((frame) => frame.slot)).toEqual([0, 3]);
  });
});
