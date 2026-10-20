import { describe, expect, it } from "vitest";
import { floodingStrategy } from "./flood";
import {
  runSimulation,
  type MeshNode,
  type PropagationStrategy,
} from "./simulation";

const line: MeshNode[] = [
  { id: "a", position: { latitude: 0, longitude: 0 } },
  { id: "b", position: { latitude: 0.0000898315, longitude: 0 } },
  { id: "c", position: { latitude: 0.000179663, longitude: 0 } },
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
  it("uses latitude and longitude in decimal degrees for meter-based range checks", () => {
    const result = runSimulation(
      {
        nodes: [
          { id: "a", position: { latitude: 47.6, longitude: -122.3 } },
          { id: "near", position: { latitude: 47.60005, longitude: -122.3 } },
          { id: "far", position: { latitude: 47.601, longitude: -122.3 } },
        ],
        sourceId: "a",
        range: 10,
        seed: 1,
      },
      floodingStrategy(0),
    );

    expect(result.reachedNodeIds).toEqual(new Set(["near"]));
  });

  it("rejects coordinates outside geographic latitude and longitude bounds", () => {
    expect(() =>
      runSimulation(
        {
          nodes: [
            { id: "a", position: { latitude: 91, longitude: 0 } },
            { id: "b", position: { latitude: 0, longitude: 0 } },
          ],
          sourceId: "a",
          range: 10,
          seed: 1,
        },
        floodingStrategy(0),
      ),
    ).toThrow("valid latitude and longitude");
  });

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
      { id: "far", position: { latitude: 0.000224579, longitude: 0 } },
    ]);

    expect(result.reachedNodeIds.size).toBe(0);
    expect(result.transmissionCount).toBe(1);
  });

  it("counts simultaneous arrivals as a receiver collision", () => {
    const nodes: MeshNode[] = [
      { id: "a", position: { latitude: 0, longitude: 0 } },
      { id: "b", position: { latitude: 0.0000898315, longitude: 0 } },
      { id: "c", position: { latitude: 0, longitude: 0.0000898315 } },
      {
        id: "d",
        position: { latitude: 0.0000898315, longitude: 0.0000898315 },
      },
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
      {
        id: "d",
        position: { latitude: 0.0000898315, longitude: 0.0000718652 },
      },
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
