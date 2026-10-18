# LoRa Propagation Lab

A browser-only mesh broadcast simulator. The first version models radio links
using node distance and a configurable range. It does not model terrain,
interference beyond same-slot collisions, fading, regional regulations, or
LoRa physical-layer airtime.

## Run locally

```sh
npm install
npm run dev
```

Open the local URL printed by Vite. To verify the app:

```sh
npm test
npm run build
```

## Try it

- Drag nodes to change the topology. Click **Add node**, then click an open
  spot on the map to place another node.
- Change the source, range, seed, or flood backoff, then run the broadcast.
- The same seed and settings produce the same backoff choices and results.
- Use **Randomize** to generate a repeatable layout from the selected seed.

## Propagation strategies

`src/simulation.ts` defines the `PropagationStrategy` and `PropagationRun`
interfaces. The included `floodingStrategy` is a demo implementation in
`src/flood.ts`; replace or extend it with a custom strategy. Strategies are
not coupled to the renderer.

The UI's **Run broadcast** handler calls `runSimulation(config, strategy)`.
Use your own `PropagationStrategy` there; the run receives the source and then
each successful `Reception`, and returns slot delays for any rebroadcasts:

```ts
const customStrategy: PropagationStrategy = {
  name: "Custom",
  createRun() {
    return {
      onStart: () => [0],
      onReception(reception, context) {
        return shouldForward(reception, context) ? [customDelaySlots] : [];
      },
    };
  },
};
```

`onStart` returns absolute initial transmit slots. `onReception` returns
non-negative slot delays (zero still schedules the next slot).

The simulator advances in discrete time slots. A receiving node decodes a
transmission only if exactly one in-range transmission is addressed to it in
that slot. Two or more simultaneous arrivals collide at that receiver. A
transmitting node cannot receive in the same slot.
