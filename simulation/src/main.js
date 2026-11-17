import {
  CHANNEL_CAPACITY,
  createSimulation,
  getChannelSummaries,
  getReachablePeers,
  stepSimulation,
} from "./simulation.js";

const svgNamespace = "http://www.w3.org/2000/svg";
const channelPalette = [
  "#858aff",
  "#53d8b5",
  "#f2b968",
  "#f18191",
  "#65b9f1",
  "#c092ff",
  "#cbd85c",
];
const initialCameraYaw = -0.55;
const initialCameraPitch = 0.48;
const minimumCameraPitch = 0.16;
const maximumCameraPitch = 1.25;
const minimumCameraZoom = 0.75;
const maximumCameraZoom = 3.5;
const layerHeight = 58;
const viewWidth = 900;
const viewHeight = 570;

const elements = {
  round: document.querySelector("#round-number"),
  status: document.querySelector("#run-status"),
  map: document.querySelector("#network-map"),
  mapNodeCount: document.querySelector("#map-node-count"),
  channelList: document.querySelector("#channel-list"),
  nodeCount: document.querySelector("#node-count"),
  nodeCountValue: document.querySelector("#node-count-value"),
  traffic: document.querySelector("#traffic-level"),
  trafficValue: document.querySelector("#traffic-value"),
  play: document.querySelector("#play-button"),
  step: document.querySelector("#step-button"),
  reset: document.querySelector("#reset-button"),
  selectedNodeTitle: document.querySelector("#selected-node-title"),
  nodeState: document.querySelector("#node-state-tag"),
  peerCount: document.querySelector("#peer-count"),
  range: document.querySelector("#range-value"),
  minimumRange: document.querySelector("#minimum-range-value"),
  rangeMeter: document.querySelector("#range-meter-fill"),
  nodeIcon: document.querySelector("#node-icon"),
  inspectorNote: document.querySelector("#inspector-note"),
  eventCount: document.querySelector("#event-count"),
  eventList: document.querySelector("#event-list"),
};

let simulation = createSimulation();
let intervalId = null;
let cameraYaw = initialCameraYaw;
let cameraPitch = initialCameraPitch;
let cameraZoom = 1;
let cameraViewBoxX = 0;
let cameraViewBoxY = 0;
let cameraDrag = null;
const seed = 43117;

function nodeColor(channelId) {
  if (channelId === "primary") return channelPalette[0];
  const shardNumber = Number(channelId.slice("shard-".length));
  return channelPalette[shardNumber % channelPalette.length];
}

function channelName(channelId) {
  if (channelId === "primary") return "Primary";
  return `Shard ${channelId.slice("shard-".length)}`;
}

function getChannelLayers() {
  const shards = new Set([
    ...simulation.shards.map((shard) => shard.id),
    ...simulation.nodes
      .map((node) => node.channelId)
      .filter((channelId) => channelId !== "primary"),
  ]);
  return [
    ...[...shards].sort(
      (a, b) => Number(a.slice("shard-".length)) - Number(b.slice("shard-".length)),
    ),
    "primary",
  ];
}

function svgElement(name, attributes = {}) {
  const element = document.createElementNS(svgNamespace, name);
  for (const [key, value] of Object.entries(attributes)) {
    element.setAttribute(key, String(value));
  }
  return element;
}

function addSvgText(parent, x, y, text, className) {
  const label = svgElement("text", { x, y, class: className });
  label.textContent = text;
  parent.append(label);
}

function circlePoints(cx, cy, radius, count = 40) {
  return Array.from({ length: count }, (_, index) => {
    const angle = (2 * Math.PI * index) / count;
    return [cx + Math.cos(angle) * radius, cy + Math.sin(angle) * radius];
  });
}

function projectScene(layers) {
  const padding = 140;
  const minWorldX = Math.min(...simulation.nodes.map((node) => node.x)) - padding;
  const maxWorldX = Math.max(...simulation.nodes.map((node) => node.x)) + padding;
  const minWorldY = Math.min(...simulation.nodes.map((node) => node.y)) - padding;
  const maxWorldY = Math.max(...simulation.nodes.map((node) => node.y)) + padding;
  const cosine = Math.cos(cameraYaw);
  const sine = Math.sin(cameraYaw);
  const horizontalScale = 0.78;
  const depthScale = Math.sin(cameraPitch) * horizontalScale;
  const heightScale = Math.cos(cameraPitch);
  const projectRaw = (x, y, layer) => ({
    x: (x * cosine - y * sine) * horizontalScale,
    y:
      (x * sine + y * cosine) * depthScale -
      layer * layerHeight * heightScale,
  });
  const corners = [
    [minWorldX, minWorldY],
    [maxWorldX, minWorldY],
    [maxWorldX, maxWorldY],
    [minWorldX, maxWorldY],
  ];
  const bounds = corners.flatMap(([x, y]) =>
    [0, layers.length - 1].map((layer) => projectRaw(x, y, layer)),
  );
  const minX = Math.min(...bounds.map((point) => point.x));
  const maxX = Math.max(...bounds.map((point) => point.x));
  const minY = Math.min(...bounds.map((point) => point.y));
  const maxY = Math.max(...bounds.map((point) => point.y));
  const scale = Math.min(
    (viewWidth - 100) / (maxX - minX),
    (viewHeight - 100) / (maxY - minY),
  );

  return {
    minWorldX,
    maxWorldX,
    minWorldY,
    maxWorldY,
    scale,
    project(x, y, layer = 0) {
      const point = projectRaw(x, y, layer);
      return {
        x: 50 + (point.x - minX) * scale,
        y: 50 + (point.y - minY) * scale,
      };
    },
  };
}

function projectedPoints(points, project, layer) {
  return points
    .map(([x, y]) => {
      const point = project(x, y, layer);
      return `${point.x},${point.y}`;
    })
    .join(" ");
}

function renderMap() {
  const layers = getChannelLayers();
  const scene = projectScene(layers);
  const viewBoxWidth = viewWidth / cameraZoom;
  const viewBoxHeight = viewHeight / cameraZoom;
  elements.map.setAttribute(
    "viewBox",
    `${cameraViewBoxX} ${cameraViewBoxY} ${viewBoxWidth} ${viewBoxHeight}`,
  );
  elements.map.replaceChildren();

  const defs = svgElement("defs");
  const grid = svgElement("pattern", {
    id: "map-grid",
    width: 26,
    height: 26,
    patternUnits: "userSpaceOnUse",
  });
  grid.append(svgElement("path", { d: "M 26 0 L 0 0 0 26", class: "grid-path" }));
  defs.append(grid);
  elements.map.append(defs);
  elements.map.append(svgElement("rect", {
    x: cameraViewBoxX,
    y: cameraViewBoxY,
    width: viewBoxWidth,
    height: viewBoxHeight,
    fill: "url(#map-grid)",
    class: "map-background",
  }));

  for (let layerIndex = 0; layerIndex < layers.length; layerIndex += 1) {
    const channelId = layers[layerIndex];
    const channelNodes = simulation.nodes.filter(
      (node) => node.channelId === channelId,
    );
    const layer = svgElement("g", {
      class: `channel-layer${channelId === "primary" ? " primary-layer" : ""}`,
      style: `--channel-color: ${nodeColor(channelId)}`,
    });
    const corners = [
      [scene.minWorldX, scene.minWorldY],
      [scene.maxWorldX, scene.minWorldY],
      [scene.maxWorldX, scene.maxWorldY],
      [scene.minWorldX, scene.maxWorldY],
    ];
    layer.append(
      svgElement("polygon", {
        points: projectedPoints(corners, scene.project, layerIndex),
        class: "channel-plane",
      }),
    );

    for (let x = scene.minWorldX + 80; x < scene.maxWorldX; x += 100) {
      const start = scene.project(x, scene.minWorldY, layerIndex);
      const end = scene.project(x, scene.maxWorldY, layerIndex);
      layer.append(
        svgElement("line", {
          x1: start.x,
          y1: start.y,
          x2: end.x,
          y2: end.y,
          class: "layer-grid",
        }),
      );
    }

    for (let y = scene.minWorldY + 80; y < scene.maxWorldY; y += 100) {
      const start = scene.project(scene.minWorldX, y, layerIndex);
      const end = scene.project(scene.maxWorldX, y, layerIndex);
      layer.append(
        svgElement("line", {
          x1: start.x,
          y1: start.y,
          x2: end.x,
          y2: end.y,
          class: "layer-grid",
        }),
      );
    }

    const clusterIds = [...new Set(channelNodes.map((node) => node.clusterId))];
    for (const clusterId of clusterIds) {
      const clusterNodes = channelNodes.filter(
        (node) => node.clusterId === clusterId,
      );
      const centerX =
        clusterNodes.reduce((sum, node) => sum + node.x, 0) / clusterNodes.length;
      const centerY =
        clusterNodes.reduce((sum, node) => sum + node.y, 0) / clusterNodes.length;
      for (const radius of [48, 78, 108]) {
        layer.append(
          svgElement("polygon", {
            points: projectedPoints(
              circlePoints(centerX, centerY, radius),
              scene.project,
              layerIndex,
            ),
            class: "terrain-contour",
          }),
        );
      }
    }

    const layerLabel = scene.project(
      scene.minWorldX + 24,
      scene.minWorldY + 18,
      layerIndex,
    );
    addSvgText(
      layer,
      layerLabel.x,
      layerLabel.y,
      `${String(layerIndex + 1).padStart(2, "0")} / ${channelName(channelId).toUpperCase()}`,
      "channel-layer-label",
    );
    elements.map.append(layer);
  }

  const selectedNode = simulation.nodes.find(
    (node) => node.id === simulation.selectedNodeId,
  );
  if (selectedNode) {
    const layerIndex = layers.indexOf(selectedNode.channelId);
    elements.map.append(
      svgElement("polygon", {
        points: projectedPoints(
          circlePoints(selectedNode.x, selectedNode.y, selectedNode.range),
          scene.project,
          layerIndex,
        ),
        class: "range-circle",
        style: `--channel-color: ${nodeColor(selectedNode.channelId)}`,
      }),
    );
  }

  for (let first = 0; first < simulation.nodes.length; first += 1) {
    for (let second = first + 1; second < simulation.nodes.length; second += 1) {
      const a = simulation.nodes[first];
      const b = simulation.nodes[second];
      if (a.clusterId !== b.clusterId || a.channelId !== b.channelId) continue;
      if (Math.hypot(a.x - b.x, a.y - b.y) > Math.max(a.range, b.range)) continue;

      const layerIndex = layers.indexOf(a.channelId);
      const start = scene.project(a.x, a.y, layerIndex);
      const end = scene.project(b.x, b.y, layerIndex);
      elements.map.append(
        svgElement("line", {
          x1: start.x,
          y1: start.y,
          x2: end.x,
          y2: end.y,
          class: `channel-link${a.channelId === "primary" ? "" : " shard-channel-link"}`,
          style: `--channel-color: ${nodeColor(a.channelId)}`,
        }),
      );
    }
  }

  for (const packet of simulation.packetEvents) {
    const sender = simulation.nodes.find((node) => node.id === packet.senderId);
    if (!sender) continue;
    const layerIndex = layers.indexOf(packet.channelId);
    for (const peer of simulation.nodes) {
      if (
        peer.id === sender.id ||
        peer.clusterId !== sender.clusterId ||
        peer.channelId !== packet.channelId ||
        Math.hypot(sender.x - peer.x, sender.y - peer.y) >
          Math.max(sender.range, peer.range)
      ) {
        continue;
      }

      const start = scene.project(sender.x, sender.y, layerIndex);
      const end = scene.project(peer.x, peer.y, layerIndex);
      elements.map.append(
        svgElement("line", {
          x1: start.x,
          y1: start.y,
          x2: end.x,
          y2: end.y,
          class: `broadcast-link${packet.success ? "" : " packet-collision"}`,
          style: `--channel-color: ${nodeColor(packet.channelId)}`,
        }),
      );
    }

    elements.map.append(
      svgElement("polygon", {
        points: projectedPoints(
          circlePoints(sender.x, sender.y, sender.range),
          scene.project,
          layerIndex,
        ),
        class: `broadcast-ripple${packet.success ? "" : " packet-collision"}`,
        style: `--channel-color: ${nodeColor(packet.channelId)}`,
      }),
    );
  }

  const displayNodes = [...simulation.nodes].sort(
    (a, b) => layers.indexOf(a.channelId) - layers.indexOf(b.channelId),
  );
  for (const node of displayNodes) {
    const layerIndex = layers.indexOf(node.channelId);
    const channelColor = nodeColor(node.channelId);
    const location = scene.project(node.x, node.y, layerIndex);
    const ground = scene.project(node.x, node.y, 0);
    const selected = node.id === simulation.selectedNodeId;
    const nodeRadius = selected ? 10 : 7;

    elements.map.append(
      svgElement("ellipse", {
        cx: ground.x,
        cy: ground.y,
        rx: 8,
        ry: 3.5,
        class: "ground-anchor",
        style: `--channel-color: ${channelColor}`,
      }),
    );

    if (layerIndex > 0) {
      elements.map.append(
        svgElement("line", {
          x1: ground.x,
          y1: ground.y,
          x2: location.x,
          y2: location.y,
          class: "elevation-stem",
          style: `--channel-color: ${channelColor}`,
        }),
      );
    }

    const group = svgElement("g", {
      class: `map-node${selected ? " is-selected" : ""}`,
      "data-node-id": node.id,
      tabindex: 0,
      role: "button",
      "aria-label": `${node.id}, ${channelName(node.channelId)}, ${getReachablePeers(simulation, node).length} peers in range`,
    });
    group.append(
      svgElement("circle", {
        cx: location.x,
        cy: location.y,
        r: nodeRadius + 6,
        class: "node-halo",
        style: `--channel-color: ${channelColor}`,
      }),
      svgElement("circle", {
        cx: location.x,
        cy: location.y,
        r: nodeRadius,
        class: "node-center-dot",
        style: `--channel-color: ${channelColor}`,
      }),
    );
    addSvgText(group, location.x, location.y + 22, node.id, "node-label");
    elements.map.append(group);
  }
}

function renderChannels() {
  const summaries = getChannelSummaries(simulation);
  elements.channelList.replaceChildren();

  if (summaries.length === 0) {
    const empty = document.createElement("p");
    empty.className = "empty-state";
    empty.textContent = "Channel activity will appear after the first round.";
    elements.channelList.append(empty);
    return;
  }

  for (const summary of summaries) {
    const utilization = (summary.attempts / CHANNEL_CAPACITY) * 100;
    const successRate =
      summary.attempts === 0
        ? null
        : Math.round((summary.successes / summary.attempts) * 100);
    const row = document.createElement("article");
    row.className = "channel-row";
    const clusterLabel = String.fromCharCode(65 + summary.clusterId);
    row.style.setProperty("--channel-color", nodeColor(summary.channelId));
    row.innerHTML = `
      <div class="channel-identity">
        <span class="channel-swatch"></span>
        <div><strong>${summary.name}</strong><small>Cluster ${clusterLabel} · ${summary.nodeCount} nodes</small></div>
      </div>
      <div class="channel-meter-wrap">
        <div class="channel-meter"><span style="width:${Math.min(100, utilization)}%"></span></div>
        <span class="channel-load">${Math.round(utilization)}%</span>
      </div>
      <div class="channel-success">${successRate === null ? "—" : `${successRate}%`}<small>success</small></div>`;
    elements.channelList.append(row);
  }
}

function renderInspector() {
  const node = simulation.nodes.find(
    (candidate) => candidate.id === simulation.selectedNodeId,
  );
  if (!node) return;

  const peers = getReachablePeers(simulation, node);
  const onShard = node.channelId !== "primary";
  elements.selectedNodeTitle.textContent = `Node ${node.id}`;
  elements.nodeState.textContent = onShard ? "ON SHARD" : "PRIMARY";
  elements.nodeState.classList.toggle("is-shard", onShard);
  elements.nodeIcon.classList.toggle("is-shard", onShard);
  elements.nodeState.style.setProperty("--channel-color", nodeColor(node.channelId));
  elements.nodeIcon.style.setProperty("--channel-color", nodeColor(node.channelId));
  elements.peerCount.textContent = String(peers.length).padStart(2, "0");
  elements.range.innerHTML = `${Math.round(node.range)} <small>units</small>`;
  elements.minimumRange.innerHTML = `${Math.round(node.minimumRange)} <small>units</small>`;
  elements.rangeMeter.style.width = `${Math.min(100, (node.range / 110) * 100)}%`;
  elements.inspectorNote.textContent = onShard
    ? "Range adapts toward the nearest-peer limit, so this node keeps a reachable neighbor."
    : "Range can adapt after joining a shard, while preserving at least one reachable peer.";
}

function renderEvents() {
  elements.eventCount.textContent = String(simulation.events.length).padStart(2, "0");
  elements.eventList.replaceChildren();

  for (const event of simulation.events) {
    const item = document.createElement("li");
    item.className = `event-item event-${event.type}`;
    const tick = document.createElement("span");
    tick.className = "event-tick";
    tick.textContent = `R${String(event.tick).padStart(3, "0")}`;
    const message = document.createElement("p");
    message.textContent = event.message;
    item.append(tick, message);
    elements.eventList.append(item);
  }
}

function render() {
  elements.round.textContent = String(simulation.tick).padStart(3, "0");
  elements.mapNodeCount.textContent = `${simulation.nodes.length} NODES`;
  elements.nodeCount.value = String(simulation.nodes.length);
  elements.nodeCountValue.value = String(simulation.nodes.length);
  elements.trafficValue.value = `${Math.round(simulation.traffic * 100)}%`;
  const running = intervalId !== null;
  elements.status.classList.toggle("is-running", running);
  elements.status.lastElementChild.textContent = running
    ? "SIMULATION RUNNING"
    : simulation.tick === 0
      ? "READY TO SIMULATE"
      : "SIMULATION PAUSED";
  renderMap();
  renderChannels();
  renderInspector();
  renderEvents();
}

function setRunning(shouldRun) {
  if (shouldRun && intervalId === null) {
    intervalId = window.setInterval(() => {
      stepSimulation(simulation);
      render();
    }, 850);
  } else if (!shouldRun && intervalId !== null) {
    window.clearInterval(intervalId);
    intervalId = null;
  }

  const running = intervalId !== null;
  elements.play.classList.toggle("is-running", running);
  elements.play.querySelector("span").textContent = running
    ? "Pause simulation"
    : "Start simulation";
  elements.step.disabled = running;
}

function resetSimulation() {
  const wasRunning = intervalId !== null;
  simulation = createSimulation({
    nodeCount: Number(elements.nodeCount.value),
    traffic: Number(elements.traffic.value) / 100,
    seed,
  });
  render();
  if (wasRunning) setRunning(true);
}

elements.play.addEventListener("click", () => {
  setRunning(intervalId === null);
});

elements.step.addEventListener("click", () => {
  stepSimulation(simulation);
  render();
});

elements.reset.addEventListener("click", resetSimulation);

elements.nodeCount.addEventListener("input", () => {
  elements.nodeCountValue.value = elements.nodeCount.value;
  resetSimulation();
});

elements.traffic.addEventListener("input", () => {
  simulation.traffic = Number(elements.traffic.value) / 100;
  elements.trafficValue.value = `${elements.traffic.value}%`;
});

elements.map.addEventListener("click", (event) => {
  const nodeGroup = event.target.closest("[data-node-id]");
  if (!nodeGroup) return;
  simulation.selectedNodeId = nodeGroup.dataset.nodeId;
  render();
});

elements.map.addEventListener("keydown", (event) => {
  if (event.key !== "Enter" && event.key !== " ") return;
  const nodeGroup = event.target.closest("[data-node-id]");
  if (!nodeGroup) return;
  event.preventDefault();
  simulation.selectedNodeId = nodeGroup.dataset.nodeId;
  render();
  elements.map.querySelector(`[data-node-id="${nodeGroup.dataset.nodeId}"]`).focus();
});

elements.map.addEventListener("pointerdown", (event) => {
  if (event.button !== 0 || event.target.closest("[data-node-id]")) return;
  cameraDrag = {
    x: event.clientX,
    y: event.clientY,
    yaw: cameraYaw,
    pitch: cameraPitch,
  };
  elements.map.setPointerCapture(event.pointerId);
  elements.map.classList.add("is-dragging");
});

elements.map.addEventListener("pointermove", (event) => {
  if (!cameraDrag) return;
  cameraYaw = cameraDrag.yaw + (event.clientX - cameraDrag.x) * 0.008;
  cameraPitch = Math.max(
    minimumCameraPitch,
    Math.min(
      maximumCameraPitch,
      cameraDrag.pitch + (event.clientY - cameraDrag.y) * 0.006,
    ),
  );
  renderMap();
});

function finishCameraDrag(event) {
  if (!cameraDrag) return;
  cameraDrag = null;
  elements.map.classList.remove("is-dragging");
  if (elements.map.hasPointerCapture(event.pointerId)) {
    elements.map.releasePointerCapture(event.pointerId);
  }
}

elements.map.addEventListener("pointerup", finishCameraDrag);
elements.map.addEventListener("pointercancel", finishCameraDrag);
elements.map.addEventListener(
  "wheel",
  (event) => {
    event.preventDefault();
    const zoomFactor = Math.exp(-event.deltaY * 0.001);
    const nextZoom = Math.max(
      minimumCameraZoom,
      Math.min(maximumCameraZoom, cameraZoom * zoomFactor),
    );
    if (nextZoom === cameraZoom) return;

    const screenMatrix = elements.map.getScreenCTM();
    if (!screenMatrix) return;
    const cursor = elements.map.createSVGPoint();
    cursor.x = event.clientX;
    cursor.y = event.clientY;
    const mapCursor = cursor.matrixTransform(screenMatrix.inverse());
    const zoomRatio = nextZoom / cameraZoom;
    cameraViewBoxX = mapCursor.x - (mapCursor.x - cameraViewBoxX) / zoomRatio;
    cameraViewBoxY = mapCursor.y - (mapCursor.y - cameraViewBoxY) / zoomRatio;
    cameraZoom = nextZoom;
    renderMap();
  },
  { passive: false },
);
document.querySelector("#reset-camera").addEventListener("click", () => {
  cameraYaw = initialCameraYaw;
  cameraPitch = initialCameraPitch;
  cameraZoom = 1;
  cameraViewBoxX = 0;
  cameraViewBoxY = 0;
  renderMap();
});

render();
