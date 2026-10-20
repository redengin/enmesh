export interface GeoPoint {
  /** Latitude in decimal degrees. */
  latitude: number;
  /** Longitude in decimal degrees. */
  longitude: number;
}

export interface MeshNode {
  id: string;
  position: GeoPoint;
}

export interface SimulationConfig {
  nodes: readonly MeshNode[];
  sourceId: string;
  range: number;
  seed: number;
  maxTransmissions?: number;
}

export interface Reception {
  packetId: string;
  senderId: string;
  receiverId: string;
  slot: number;
  hop: number;
}

export interface Transmission {
  packetId: string;
  senderId: string;
  slot: number;
  hop: number;
}

export interface PropagationContext {
  readonly packetId: string;
  readonly sourceId: string;
  readonly nodes: readonly MeshNode[];
  readonly range: number;
  readonly random: () => number;
}

export interface PropagationRun {
  /** Return absolute transmit slots for the source; slot zero starts immediately. */
  onStart(context: PropagationContext): readonly number[];
  /** Return slot delays; a zero delay still waits until the next slot. */
  onReception(
    reception: Reception,
    context: PropagationContext,
  ): readonly number[];
}

export interface PropagationStrategy {
  readonly name: string;
  createRun(): PropagationRun;
}

export interface SimulationFrame {
  slot: number;
  transmissions: readonly Transmission[];
  receptions: readonly Reception[];
  collidedReceivers: readonly string[];
}

export interface SimulationResult {
  frames: readonly SimulationFrame[];
  reachedNodeIds: ReadonlySet<string>;
  transmissionCount: number;
  successfulReceptionCount: number;
  collisionCount: number;
  maxHop: number;
}

const DEFAULT_MAX_TRANSMISSIONS = 1_000;
const MAX_ALLOWED_TRANSMISSIONS = 10_000;
const PACKET_ID = "broadcast-1";
const EARTH_RADIUS_METERS = 6_371_000;

export function seededRandom(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let value = state;
    value = Math.imul(value ^ (value >>> 15), value | 1);
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
    return ((value ^ (value >>> 14)) >>> 0) / 4_294_967_296;
  };
}

function distance(a: GeoPoint, b: GeoPoint): number {
  const latitudeA = (a.latitude * Math.PI) / 180;
  const latitudeB = (b.latitude * Math.PI) / 180;
  const latitudeDelta = latitudeB - latitudeA;
  const longitudeDelta = ((b.longitude - a.longitude) * Math.PI) / 180;
  const haversine = Math.min(
    1,
    Math.sin(latitudeDelta / 2) ** 2 +
      Math.cos(latitudeA) *
        Math.cos(latitudeB) *
        Math.sin(longitudeDelta / 2) ** 2,
  );
  return 2 * EARTH_RADIUS_METERS * Math.asin(Math.sqrt(haversine));
}

function validateConfig(config: SimulationConfig): void {
  if (!Number.isFinite(config.range) || config.range < 0) {
    throw new RangeError("Radio range must be a finite, non-negative number.");
  }
  if (!Number.isSafeInteger(config.seed)) {
    throw new RangeError("Seed must be a safe integer.");
  }
  const ids = new Set<string>();
  for (const node of config.nodes) {
    if (!node.id || ids.has(node.id)) {
      throw new Error("Node IDs must be non-empty and unique.");
    }
    if (
      !Number.isFinite(node.position.latitude) ||
      !Number.isFinite(node.position.longitude) ||
      node.position.latitude < -90 ||
      node.position.latitude > 90 ||
      node.position.longitude < -180 ||
      node.position.longitude > 180
    ) {
      throw new Error(
        `Node ${node.id} must have valid latitude and longitude coordinates.`,
      );
    }
    ids.add(node.id);
  }
  if (!ids.has(config.sourceId)) {
    throw new Error(`Source node ${config.sourceId} does not exist.`);
  }
  const maxTransmissions = config.maxTransmissions ?? DEFAULT_MAX_TRANSMISSIONS;
  if (
    !Number.isInteger(maxTransmissions) ||
    maxTransmissions < 1 ||
    maxTransmissions > MAX_ALLOWED_TRANSMISSIONS
  ) {
    throw new RangeError(
      `Maximum transmissions must be between 1 and ${MAX_ALLOWED_TRANSMISSIONS}.`,
    );
  }
}

export function runSimulation(
  config: SimulationConfig,
  strategy: PropagationStrategy,
): SimulationResult {
  validateConfig(config);

  const nodesById = new Map(config.nodes.map((node) => [node.id, node]));
  const context: PropagationContext = {
    packetId: PACKET_ID,
    sourceId: config.sourceId,
    nodes: config.nodes,
    range: config.range,
    random: seededRandom(config.seed),
  };
  const run = strategy.createRun();
  const scheduled = new Map<number, Transmission[]>();
  const maxTransmissions = config.maxTransmissions ?? DEFAULT_MAX_TRANSMISSIONS;
  const frames: SimulationFrame[] = [];
  const reachedNodeIds = new Set<string>();
  const hopByNode = new Map<string, number>([[config.sourceId, 0]]);
  let transmissionCount = 0;
  let successfulReceptionCount = 0;
  let collisionCount = 0;

  const startPlans = run.onStart(context);
  for (const delay of startPlans) {
    if (!Number.isSafeInteger(delay) || delay < 0) {
      throw new RangeError("Strategy delays must be non-negative integers.");
    }
    const slot = delay;
    const transmissions = scheduled.get(slot) ?? [];
    transmissions.push({
      packetId: context.packetId,
      senderId: config.sourceId,
      slot,
      hop: 0,
    });
    scheduled.set(slot, transmissions);
  }

  while (scheduled.size > 0) {
    if (transmissionCount >= maxTransmissions) {
      throw new Error(
        `Simulation exceeded the ${maxTransmissions} transmission limit.`,
      );
    }

    const slot = Math.min(...scheduled.keys());
    const transmissions = scheduled.get(slot);
    if (!transmissions) {
      throw new Error(`Missing scheduled transmissions for slot ${slot}.`);
    }
    if (transmissionCount + transmissions.length > maxTransmissions) {
      throw new Error(
        `Simulation exceeded the ${maxTransmissions} transmission limit.`,
      );
    }
    scheduled.delete(slot);

    const transmitters = new Set(transmissions.map((tx) => tx.senderId));
    const arrivalsByReceiver = new Map<string, Reception[]>();
    for (const transmission of transmissions) {
      const sender = nodesById.get(transmission.senderId);
      if (!sender) {
        throw new Error(`Unknown transmitting node ${transmission.senderId}.`);
      }
      for (const receiver of config.nodes) {
        if (
          receiver.id === sender.id ||
          transmitters.has(receiver.id) ||
          distance(sender.position, receiver.position) > config.range
        ) {
          continue;
        }
        const arrivals = arrivalsByReceiver.get(receiver.id) ?? [];
        arrivals.push({
          packetId: transmission.packetId,
          senderId: transmission.senderId,
          receiverId: receiver.id,
          slot,
          hop: transmission.hop + 1,
        });
        arrivalsByReceiver.set(receiver.id, arrivals);
      }
    }

    const receptions: Reception[] = [];
    const collidedReceivers: string[] = [];
    for (const [receiverId, arrivals] of arrivalsByReceiver) {
      if (arrivals.length === 1) {
        receptions.push(arrivals[0]!);
      } else {
        collidedReceivers.push(receiverId);
      }
    }
    receptions.sort((a, b) => a.receiverId.localeCompare(b.receiverId));
    collidedReceivers.sort();

    transmissionCount += transmissions.length;
    successfulReceptionCount += receptions.length;
    collisionCount += collidedReceivers.length;
    for (const reception of receptions) {
      if (reception.receiverId !== config.sourceId) {
        reachedNodeIds.add(reception.receiverId);
      }
      const previousHop = hopByNode.get(reception.receiverId);
      if (previousHop === undefined || reception.hop < previousHop) {
        hopByNode.set(reception.receiverId, reception.hop);
      }
    }
    frames.push({ slot, transmissions, receptions, collidedReceivers });

    for (const reception of receptions) {
      const delays = run.onReception(reception, context);
      for (const delay of delays) {
        if (!Number.isSafeInteger(delay) || delay < 0) {
          throw new RangeError("Strategy delays must be non-negative integers.");
        }
        const nextSlot = slot + Math.max(1, delay);
        const planned = scheduled.get(nextSlot) ?? [];
        planned.push({
          packetId: reception.packetId,
          senderId: reception.receiverId,
          slot: nextSlot,
          hop: reception.hop,
        });
        scheduled.set(nextSlot, planned);
      }
    }
  }

  return {
    frames,
    reachedNodeIds,
    transmissionCount,
    successfulReceptionCount,
    collisionCount,
    maxHop: Math.max(0, ...hopByNode.values()),
  };
}
