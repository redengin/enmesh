export const CHANNEL_CAPACITY = 4;

const MAX_NODES = 48;
const CLUSTER_SIZE = 8;
const MAX_RANGE = 104;
const CONGESTION_THRESHOLD = 0.68;
const CONGESTION_ROUNDS = 3;

function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}

function random(state) {
  state.randomState = (state.randomState + 0x6d2b79f5) | 0;
  let value = state.randomState;
  value = Math.imul(value ^ (value >>> 15), value | 1);
  value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
  return ((value ^ (value >>> 14)) >>> 0) / 4294967296;
}

function distance(a, b) {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

function appendEvent(state, message, type = "info") {
  state.events.unshift({ tick: state.tick, message, type });
  state.events.length = Math.min(state.events.length, 8);
}

function createNodes(nodeCount) {
  const clusterCount = Math.ceil(nodeCount / CLUSTER_SIZE);
  const baseSize = Math.floor(nodeCount / clusterCount);
  const extraNodes = nodeCount % clusterCount;
  const clusterColumns = Math.ceil(Math.sqrt(clusterCount));
  const nodes = [];
  let id = 1;

  for (let clusterId = 0; clusterId < clusterCount; clusterId += 1) {
    const clusterSize = baseSize + (clusterId < extraNodes ? 1 : 0);
    const centerX = 150 + (clusterId % clusterColumns) * 250;
    const centerY = 150 + Math.floor(clusterId / clusterColumns) * 250;
    const clusterNodes = [];

    for (let index = 0; index < clusterSize; index += 1) {
      const angle = (2 * Math.PI * index) / clusterSize - Math.PI / 2;
      clusterNodes.push({
        id: `N${String(id).padStart(2, "0")}`,
        clusterId,
        x: centerX + Math.cos(angle) * 32,
        y: centerY + Math.sin(angle) * 32,
        channelId: "primary",
        range: MAX_RANGE,
        minimumRange: MAX_RANGE,
        poorRounds: 0,
        lastMovedAt: -1,
        lastAttempt: false,
        lastSuccess: false,
      });
      id += 1;
    }

    for (const node of clusterNodes) {
      const nearestPeerDistance = Math.min(
        ...clusterNodes
          .filter((peer) => peer.id !== node.id)
          .map((peer) => distance(node, peer)),
      );
      node.minimumRange = Math.ceil(nearestPeerDistance + 2);
    }

    nodes.push(...clusterNodes);
  }

  return nodes;
}

function channelName(channelId) {
  if (channelId === "primary") return "Primary";
  return `Shard ${channelId.slice("shard-".length)}`;
}

export function createSimulation({
  nodeCount = 24,
  traffic = 0.8,
  seed = 43117,
} = {}) {
  const safeNodeCount = Math.round(clamp(nodeCount, 2, MAX_NODES));
  const nodes = createNodes(safeNodeCount);
  const clusterCount = Math.max(...nodes.map((node) => node.clusterId)) + 1;

  return {
    tick: 0,
    traffic: clamp(traffic, 0.05, 1),
    seed: seed >>> 0,
    randomState: seed >>> 0,
    nodes,
    shards: [],
    overloadRounds: Array(clusterCount).fill(0),
    packetEvents: [],
    lastStats: [],
    events: [
      {
        tick: 0,
        message: `${safeNodeCount} nodes joined the primary channel across ${clusterCount} local cluster${clusterCount === 1 ? "" : "s"}.`,
        type: "info",
      },
    ],
    selectedNodeId: nodes[0].id,
  };
}

export function getReachablePeers(state, node) {
  return state.nodes.filter(
    (peer) => peer.id !== node.id && distance(node, peer) <= node.range,
  );
}

export function getChannelSummaries(state) {
  const channels = new Map();

  for (const node of state.nodes) {
    const key = `${node.clusterId}:${node.channelId}`;
    if (!channels.has(key)) {
      channels.set(key, {
        clusterId: node.clusterId,
        channelId: node.channelId,
        name: channelName(node.channelId),
        nodeCount: 0,
        attempts: 0,
        successes: 0,
      });
    }
    channels.get(key).nodeCount += 1;
  }

  for (const stat of state.lastStats) {
    const summary = channels.get(`${stat.clusterId}:${stat.channelId}`);
    if (summary) {
      summary.attempts = stat.attempts;
      summary.successes = stat.successes;
    }
  }

  return [...channels.values()].sort(
    (a, b) =>
      a.clusterId - b.clusterId ||
      (a.channelId === "primary" ? -1 : b.channelId === "primary" ? 1 : 0),
  );
}

export function stepSimulation(state) {
  state.tick += 1;
  state.packetEvents = [];

  const channelAttempts = new Map();
  for (const node of state.nodes) {
    node.lastAttempt = random(state) < state.traffic;
    node.lastSuccess = false;
    if (!node.lastAttempt) continue;

    const key = `${node.clusterId}:${node.channelId}`;
    if (!channelAttempts.has(key)) channelAttempts.set(key, []);
    channelAttempts.get(key).push(node);
  }

  const statsByChannel = new Map();
  for (const node of state.nodes) {
    const key = `${node.clusterId}:${node.channelId}`;
    if (!statsByChannel.has(key)) {
      statsByChannel.set(key, {
        clusterId: node.clusterId,
        channelId: node.channelId,
        attempts: 0,
        successes: 0,
        capacity: CHANNEL_CAPACITY,
      });
    }
  }

  for (const shard of state.shards) {
    const key = `${shard.clusterId}:${shard.id}`;
    if (!statsByChannel.has(key)) {
      statsByChannel.set(key, {
        clusterId: shard.clusterId,
        channelId: shard.id,
        attempts: 0,
        successes: 0,
        capacity: CHANNEL_CAPACITY,
      });
    }
  }

  for (const [key, attemptedNodes] of channelAttempts) {
    const successRate = Math.min(1, CHANNEL_CAPACITY / attemptedNodes.length);
    const stat = statsByChannel.get(key);
    stat.attempts = attemptedNodes.length;

    for (const node of attemptedNodes) {
      node.lastSuccess = random(state) < successRate;
      if (node.lastSuccess) stat.successes += 1;

      state.packetEvents.push({
        senderId: node.id,
        channelId: node.channelId,
        success: node.lastSuccess,
      });
    }
  }

  state.lastStats = [...statsByChannel.values()];
  const clusterCount = state.overloadRounds.length;

  for (let clusterId = 0; clusterId < clusterCount; clusterId += 1) {
    const primaryStat = statsByChannel.get(`${clusterId}:primary`);
    const channelIsCongested =
      primaryStat &&
      primaryStat.attempts / CHANNEL_CAPACITY > 1 / CONGESTION_THRESHOLD;
    for (const node of state.nodes) {
      if (node.clusterId === clusterId && node.channelId === "primary") {
        node.poorRounds = channelIsCongested ? node.poorRounds + 1 : 0;
      }
    }

    if (!channelIsCongested) {
      state.overloadRounds[clusterId] = 0;
    } else {
      state.overloadRounds[clusterId] += 1;
    }

    if (
      state.overloadRounds[clusterId] >= CONGESTION_ROUNDS &&
      !state.shards.some((shard) => shard.clusterId === clusterId)
    ) {
      const advertiser = state.nodes
        .filter(
          (node) => node.clusterId === clusterId && node.channelId === "primary",
        )
        .sort((a, b) => b.poorRounds - a.poorRounds || a.id.localeCompare(b.id))[0];
      const shard = {
        id: `shard-${clusterId + 1}`,
        clusterId,
        advertiserId: advertiser.id,
        createdAt: state.tick,
      };
      state.shards.push(shard);
      appendEvent(
        state,
        `Node ${advertiser.id} advertised ${channelName(shard.id)} after repeated primary-channel contention.`,
        "shard",
      );
    }

    const shard = state.shards.find((entry) => entry.clusterId === clusterId);
    if (!shard) continue;

    const localStats = [...statsByChannel.values()].filter(
      (stat) => stat.clusterId === clusterId,
    );
    const overloaded = localStats
      .filter((stat) => stat.attempts > stat.capacity)
      .sort((a, b) => b.attempts - a.attempts)[0];
    if (overloaded) {
      const destination = localStats
        .filter(
          (stat) =>
            stat.channelId !== overloaded.channelId &&
            stat.attempts < stat.capacity,
        )
        .sort((a, b) => a.attempts - b.attempts)[0];
      if (destination) {
        const migratingNode = state.nodes
          .filter(
            (node) =>
              node.clusterId === clusterId &&
              node.channelId === overloaded.channelId,
          )
          .sort((a, b) => a.lastMovedAt - b.lastMovedAt || a.id.localeCompare(b.id))[0];
        migratingNode.channelId = destination.channelId;
        migratingNode.lastMovedAt = state.tick;
        appendEvent(
          state,
          `Node ${migratingNode.id} joined ${channelName(destination.channelId)} to ease local contention.`,
          "move",
        );
      }
    }
  }

  for (const node of state.nodes) {
    if (
      node.channelId !== "primary" &&
      state.tick % 3 === 0 &&
      node.range > node.minimumRange
    ) {
      node.range = Math.max(node.minimumRange, node.range - 5);
      if (node.range === node.minimumRange) {
        appendEvent(
          state,
          `Node ${node.id} reduced transmit range to its nearest-peer safety limit.`,
          "power",
        );
      }
    }
  }

  return state;
}
