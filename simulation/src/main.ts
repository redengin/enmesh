import "./style.css";
import { floodingStrategy } from "./flood";
import {
  runSimulation,
  seededRandom,
  type MeshNode,
  type Point,
  type SimulationFrame,
  type SimulationResult,
} from "./simulation";

const BASE_WIDTH = 1_000;
const BASE_HEIGHT = 600;
function requiredApp(): HTMLDivElement {
  const element = document.querySelector<HTMLDivElement>("#app");
  if (!element) throw new Error("App root element #app was not found.");
  return element;
}

const app = requiredApp();

app.innerHTML = `
  <main class="app-shell">
    <header class="topbar">
      <a class="brand" href="#" aria-label="Enmesh propagation lab home">
        <span class="brand-mark" aria-hidden="true"><span></span><span></span><span></span></span>
        <span class="brand-name">enmesh<span class="brand-divider">/</span>lab</span>
      </a>
      <div class="topbar-context">
        <span class="context-dot"></span>
        <span>RADIO NETWORK SANDBOX</span>
      </div>
      <a class="github-link" href="https://github.com/redengin/enmesh" target="_blank" rel="noreferrer">PROJECT <span aria-hidden="true">↗</span></a>
    </header>

    <section class="intro">
      <div>
        <p class="eyebrow"><span>01</span> PROPAGATION STUDY</p>
        <h1>LoRa propagation <span>lab.</span></h1>
        <p class="intro-copy">Shape a mesh. Send a broadcast. See what makes it through.</p>
      </div>
      <div class="model-note"><span class="note-icon" aria-hidden="true">i</span><span>Range-based model <b>·</b> discrete airtime slots</span></div>
    </section>

    <section class="workspace" aria-label="Mesh simulation workspace">
      <aside class="control-panel">
        <div class="panel-heading">
          <div>
            <p class="section-kicker">EXPERIMENT SETUP</p>
            <h2>Network</h2>
          </div>
          <span class="live-badge"><span></span> READY</span>
        </div>

        <div class="control-section">
          <div class="section-label"><span>01</span> TOPOLOGY</div>
          <div class="field-row seed-row">
            <label for="seed-input">Layout seed</label>
            <div class="seed-control">
              <input id="seed-input" type="number" value="42" step="1" aria-label="Layout and simulation seed" />
              <button id="randomize-button" class="icon-button" type="button" title="Generate repeatable layout from seed" aria-label="Randomize layout">↻</button>
            </div>
          </div>
          <div class="field-row">
            <label for="count-input">Node count</label>
            <div class="compact-input">
              <input id="count-input" type="number" value="18" min="2" max="60" step="1" />
              <span>NODES</span>
            </div>
          </div>
          <button id="add-node-button" class="quiet-button" type="button" aria-pressed="false"><span aria-hidden="true">＋</span> Add node</button>
          <div id="node-list" class="node-list" aria-label="Network nodes"></div>
        </div>

        <div class="control-section">
          <div class="section-label"><span>02</span> RADIO</div>
          <div class="range-heading">
            <label for="range-input">Broadcast range</label>
            <output id="range-value" for="range-input">180 m</output>
          </div>
          <input id="range-input" class="range-slider" type="range" min="60" max="320" value="180" />
          <div class="range-scale"><span>60 m</span><span>320 m</span></div>
          <p class="field-help">Nodes within range can hear each transmission.</p>
        </div>

        <div class="control-section">
          <div class="section-label"><span>03</span> PROPAGATION</div>
          <div class="field-row">
            <label for="source-select">Broadcast source</label>
            <select id="source-select"></select>
          </div>
          <div class="field-row backoff-row">
            <label for="backoff-input">Flood backoff</label>
            <div class="compact-input">
              <input id="backoff-input" type="number" value="2" min="0" max="12" step="1" />
              <span>SLOTS</span>
            </div>
          </div>
          <p class="field-help">Random delay before a newly reached node rebroadcasts.</p>
        </div>

        <button id="run-button" class="run-button" type="button"><span class="play-icon" aria-hidden="true">▶</span> Run broadcast <span class="run-shortcut">⏎</span></button>
        <p id="status-message" class="status-message" role="status" aria-live="polite">Drag nodes to shape your network.</p>
      </aside>

      <section class="map-panel" aria-label="Interactive network map">
        <div class="map-toolbar">
          <div class="map-title">
            <span class="map-title-mark" aria-hidden="true">⌘</span>
            <span>NETWORK MAP</span>
            <span class="node-count-label" id="map-node-count">18 NODES</span>
          </div>
          <div class="map-actions"><span class="drag-hint"><span aria-hidden="true">↔</span> DRAG TO REPOSITION</span><button id="reset-run-button" class="reset-button" type="button">RESET RUN</button></div>
        </div>
        <div class="canvas-wrap">
          <canvas id="network-canvas" aria-label="Network topology. Drag nodes to reposition them."></canvas>
          <div id="empty-hint" class="empty-hint" hidden>Use <b>Add node</b> to begin building your network.</div>
          <div class="map-coordinates" aria-hidden="true"><span>LOCAL FIELD · 1.0 × 0.6 KM</span><span>SIMULATION VIEW</span></div>
        </div>
        <div class="map-legend">
          <span><i class="legend-dot source"></i> SOURCE</span>
          <span><i class="legend-dot reachable"></i> REACHED</span>
          <span><i class="legend-dot waiting"></i> WAITING</span>
          <span><i class="legend-line"></i> IN RANGE</span>
        </div>
      </section>

      <aside class="results-panel">
        <div class="panel-heading">
          <div>
            <p class="section-kicker">LIVE ANALYSIS</p>
            <h2>Run results</h2>
          </div>
          <span id="result-state" class="result-state"><i></i> IDLE</span>
        </div>
        <div class="reach-card">
          <div class="reach-card-top"><span>DELIVERY REACH</span><span id="reach-fraction">— / —</span></div>
          <div id="reach-percent" class="reach-percent">—<span>%</span></div>
          <div class="reach-track"><div id="reach-bar"></div></div>
          <p id="reach-caption">Run a broadcast to see how far your message travels.</p>
        </div>
        <div class="metric-grid">
          <div class="metric"><span>TRANSMISSIONS</span><b id="tx-count">—</b><small>broadcast attempts</small></div>
          <div class="metric"><span>COLLISIONS</span><b id="collision-count">—</b><small>receiver / slot events</small></div>
          <div class="metric"><span>MAX HOPS</span><b id="hop-count">—</b><small>from source</small></div>
          <div class="metric"><span>RECEIVED</span><b id="rx-count">—</b><small>successful decodes</small></div>
        </div>
        <div class="event-log">
          <div class="section-label"><span>↳</span> TRANSMISSION LOG</div>
          <div id="event-list" class="event-list"><p class="log-placeholder">Awaiting simulation data…</p></div>
        </div>
        <div class="assumption-note"><span aria-hidden="true">⌁</span><p><b>SIMPLIFIED RF MODEL</b> Collisions occur when multiple neighbors transmit in the same slot. Terrain, fading, and airtime physics are not modeled.</p></div>
      </aside>
    </section>
    <footer class="page-footer"><span>ENMESH PROPAGATION LAB <b>·</b> EXPERIMENTAL</span><span>DETERMINISTIC SEEDING ENABLED</span></footer>
  </main>
`;

function requiredElement<T extends Element>(selector: string): T {
  const element = app.querySelector<T>(selector);
  if (!element) throw new Error(`Required UI element ${selector} was not found.`);
  return element;
}

function requiredCanvasContext(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
  const canvasContext = canvas.getContext("2d");
  if (!canvasContext) throw new Error("The network canvas does not support 2D rendering.");
  return canvasContext;
}

const canvas = requiredElement<HTMLCanvasElement>("#network-canvas");
const context = requiredCanvasContext(canvas);
const seedInput = requiredElement<HTMLInputElement>("#seed-input");
const countInput = requiredElement<HTMLInputElement>("#count-input");
const rangeInput = requiredElement<HTMLInputElement>("#range-input");
const rangeValue = requiredElement<HTMLOutputElement>("#range-value");
const sourceSelect = requiredElement<HTMLSelectElement>("#source-select");
const backoffInput = requiredElement<HTMLInputElement>("#backoff-input");
const nodeList = requiredElement<HTMLDivElement>("#node-list");
const addNodeButton = requiredElement<HTMLButtonElement>("#add-node-button");
const statusMessage = requiredElement<HTMLParagraphElement>("#status-message");
const runButton = requiredElement<HTMLButtonElement>("#run-button");
const resultState = requiredElement<HTMLSpanElement>("#result-state");
const eventList = requiredElement<HTMLDivElement>("#event-list");
const emptyHint = requiredElement<HTMLDivElement>("#empty-hint");

let nodes: MeshNode[] = [];
let sourceId = "";
let reachedIds = new Set<string>();
let activeTransmissions = new Set<string>();
let activeReceivers = new Set<string>();
let collidedReceivers = new Set<string>();
let dragNodeId: string | undefined;
let runTimer: number | undefined;
let hasRun = false;
let placingNode = false;

function parseInteger(input: HTMLInputElement, label: string, min: number, max: number): number {
  const value = Number(input.value);
  if (!Number.isInteger(value) || value < min || value > max) {
    throw new RangeError(`${label} must be a whole number from ${min} to ${max}.`);
  }
  return value;
}

function getSeed(): number {
  return parseInteger(seedInput, "Seed", -2_147_483_648, 2_147_483_647);
}

function generateNodes(count: number, seed: number): MeshNode[] {
  const random = seededRandom(seed);
  const result: MeshNode[] = [];
  for (let i = 0; i < count; i += 1) {
    result.push({
      id: `N${i + 1}`,
      position: {
        x: 55 + random() * (BASE_WIDTH - 110),
        y: 48 + random() * (BASE_HEIGHT - 96),
      },
    });
  }
  return result;
}

function stopRun(): void {
  if (runTimer !== undefined) {
    window.clearTimeout(runTimer);
    runTimer = undefined;
  }
}

function clearRun(): void {
  stopRun();
  reachedIds = new Set();
  activeTransmissions = new Set();
  activeReceivers = new Set();
  collidedReceivers = new Set();
  hasRun = false;
  statusMessage.textContent = "Drag nodes to shape your network.";
  resultState.className = "result-state";
  resultState.innerHTML = "<i></i> IDLE";
  eventList.innerHTML = '<p class="log-placeholder">Awaiting simulation data…</p>';
  app.querySelector("#reach-fraction")!.textContent = "— / —";
  app.querySelector("#reach-percent")!.innerHTML = "—<span>%</span>";
  app.querySelector("#reach-bar")!.setAttribute("style", "width: 0%");
  app.querySelector("#reach-caption")!.textContent =
    "Run a broadcast to see how far your message travels.";
  for (const id of ["tx-count", "collision-count", "hop-count", "rx-count"]) {
    app.querySelector(`#${id}`)!.textContent = "—";
  }
  renderNodeControls();
  render();
}

function addLogEntry(frame: SimulationFrame): void {
  const entry = document.createElement("div");
  entry.className = "log-entry";
  const txCount = frame.transmissions.length;
  const rxCount = frame.receptions.length;
  const collidedCount = frame.collidedReceivers.length;
  const summary =
    collidedCount > 0
      ? `${collidedCount} collision${collidedCount === 1 ? "" : "s"}`
      : `${rxCount} decode${rxCount === 1 ? "" : "s"}`;
  entry.innerHTML = `<span class="log-slot">S${frame.slot.toString().padStart(2, "0")}</span><span class="log-summary">${txCount} TX <i>·</i> ${summary}</span><span class="log-mark ${collidedCount > 0 ? "collision" : "success"}">${collidedCount > 0 ? "!" : "✓"}</span>`;
  eventList.prepend(entry);
  while (eventList.children.length > 7) eventList.lastElementChild?.remove();
}

function renderResults(result: SimulationResult): void {
  const recipients = Math.max(0, nodes.length - 1);
  const reached = result.reachedNodeIds.size;
  const percent = recipients === 0 ? 0 : Math.round((reached / recipients) * 100);
  app.querySelector("#reach-fraction")!.textContent = `${reached} / ${recipients}`;
  app.querySelector("#reach-percent")!.innerHTML = `${percent}<span>%</span>`;
  app.querySelector("#reach-bar")!.setAttribute("style", `width: ${percent}%`);
  app.querySelector("#reach-caption")!.textContent =
    reached === recipients
      ? "Every other node decoded the broadcast."
      : `${recipients - reached} node${recipients - reached === 1 ? "" : "s"} did not decode the broadcast.`;
  app.querySelector("#tx-count")!.textContent = String(result.transmissionCount);
  app.querySelector("#collision-count")!.textContent = String(result.collisionCount);
  app.querySelector("#hop-count")!.textContent = String(result.maxHop);
  app.querySelector("#rx-count")!.textContent = String(result.successfulReceptionCount);
}

function playFrames(frames: readonly SimulationFrame[], result: SimulationResult, index = 0): void {
  if (index >= frames.length) {
    activeTransmissions = new Set();
    activeReceivers = new Set();
    collidedReceivers = new Set();
    resultState.className = "result-state complete";
    resultState.innerHTML = "<i></i> COMPLETE";
    statusMessage.textContent = "Broadcast complete. Adjust the topology or settings and run again.";
    render();
    return;
  }

  const frame = frames[index];
  if (!frame) return;
  activeTransmissions = new Set(frame.transmissions.map((tx) => tx.senderId));
  activeReceivers = new Set(frame.receptions.map((rx) => rx.receiverId));
  collidedReceivers = new Set(frame.collidedReceivers);
  for (const id of activeReceivers) reachedIds.add(id);
  addLogEntry(frame);
  renderNodeControls();
  render();
  runTimer = window.setTimeout(
    () => playFrames(frames, result, index + 1),
    620,
  );
}

function runBroadcast(): void {
  setPlacementMode(false);
  clearRun();
  try {
    if (nodes.length < 2) {
      throw new Error("Add at least two nodes before running a broadcast.");
    }
    const seed = getSeed();
    const backoff = parseInteger(backoffInput, "Flood backoff", 0, 12);
    const range = Number(rangeInput.value);
    const result = runSimulation(
      { nodes, sourceId, range, seed },
      floodingStrategy(backoff),
    );
    hasRun = true;
    renderResults(result);
    resultState.className = "result-state running";
    resultState.innerHTML = "<i></i> RUNNING";
    statusMessage.textContent = "Broadcast in progress…";
    playFrames(result.frames, result);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    statusMessage.textContent = message;
    resultState.className = "result-state error";
    resultState.innerHTML = "<i></i> ERROR";
  }
}

function renderNodeControls(): void {
  nodeList.innerHTML = "";
  sourceSelect.innerHTML = "";
  for (const node of nodes) {
    const option = document.createElement("option");
    option.value = node.id;
    option.textContent = node.id;
    sourceSelect.append(option);

    const row = document.createElement("div");
    row.className = `node-row${sourceId === node.id ? " is-source" : ""}`;
    const status = sourceId === node.id ? "SOURCE" : reachedIds.has(node.id) ? "REACHED" : "NODE";
    row.innerHTML = `<span class="node-row-status ${sourceId === node.id ? "source" : reachedIds.has(node.id) ? "reached" : ""}"></span><span class="node-row-name">${node.id}</span><span class="node-row-tag">${status}</span>`;
    const useButton = document.createElement("button");
    useButton.className = "node-source-button";
    useButton.type = "button";
    useButton.textContent = sourceId === node.id ? "SOURCE" : "Set source";
    useButton.setAttribute("aria-label", `Set ${node.id} as broadcast source`);
    useButton.disabled = sourceId === node.id;
    useButton.addEventListener("click", () => {
      sourceId = node.id;
      clearRun();
      renderNodeControls();
      render();
    });
    const removeButton = document.createElement("button");
    removeButton.className = "node-remove-button";
    removeButton.type = "button";
    removeButton.textContent = "×";
    removeButton.title = `Remove ${node.id}`;
    removeButton.setAttribute("aria-label", `Remove ${node.id}`);
    removeButton.disabled = nodes.length <= 2;
    removeButton.addEventListener("click", () => removeNode(node.id));
    row.append(useButton, removeButton);
    nodeList.append(row);
  }
  sourceSelect.value = sourceId;
  app.querySelector("#map-node-count")!.textContent = `${nodes.length} NODES`;
  addNodeButton.disabled = nodes.length >= 60;
  emptyHint.hidden = nodes.length !== 0;
}

function removeNode(nodeId: string): void {
  if (nodes.length <= 2) return;
  nodes = nodes.filter((node) => node.id !== nodeId);
  if (sourceId === nodeId) sourceId = nodes[0]?.id ?? "";
  clearRun();
  renderNodeControls();
  render();
}

function randomize(): void {
  try {
    const count = parseInteger(countInput, "Node count", 2, 60);
    nodes = generateNodes(count, getSeed());
    sourceId = nodes[0]?.id ?? "";
    setPlacementMode(false);
    clearRun();
    renderNodeControls();
    statusMessage.textContent = "New repeatable layout generated from the seed.";
    render();
  } catch (error) {
    statusMessage.textContent = error instanceof Error ? error.message : String(error);
  }
}

function setPlacementMode(enabled: boolean): void {
  placingNode = enabled;
  canvas.classList.toggle("is-placing", enabled);
  addNodeButton.classList.toggle("is-active", enabled);
  addNodeButton.setAttribute("aria-pressed", String(enabled));
  addNodeButton.innerHTML = enabled
    ? '<span aria-hidden="true">⌖</span> Click map to place node'
    : '<span aria-hidden="true">＋</span> Add node';
}

function addNode(): void {
  if (nodes.length >= 60) {
    statusMessage.textContent = "The simulator supports up to 60 nodes.";
    return;
  }
  setPlacementMode(!placingNode);
  statusMessage.textContent = placingNode
    ? "Click an open spot on the map to place a node."
    : "Node placement canceled.";
}

function placeNode(point: Point): void {
  const nextId = `N${Math.max(0, ...nodes.map((node) => Number(node.id.slice(1)) || 0)) + 1}`;
  nodes = [
    ...nodes,
    {
      id: nextId,
      position: {
        x: Math.min(BASE_WIDTH - 24, Math.max(24, point.x)),
        y: Math.min(BASE_HEIGHT - 24, Math.max(24, point.y)),
      },
    },
  ];
  setPlacementMode(false);
  clearRun();
  renderNodeControls();
  statusMessage.textContent = `${nextId} placed. Drag it to shape the topology.`;
  render();
}

function canvasPoint(event: PointerEvent): Point {
  const bounds = canvas.getBoundingClientRect();
  return {
    x: ((event.clientX - bounds.left) / bounds.width) * BASE_WIDTH,
    y: ((event.clientY - bounds.top) / bounds.height) * BASE_HEIGHT,
  };
}

function nodeAt(point: Point): MeshNode | undefined {
  return [...nodes]
    .reverse()
    .find((node) => Math.hypot(node.position.x - point.x, node.position.y - point.y) < 28);
}

function setCanvasScale(): void {
  const bounds = canvas.getBoundingClientRect();
  const ratio = window.devicePixelRatio || 1;
  canvas.width = Math.max(1, Math.round(bounds.width * ratio));
  canvas.height = Math.max(1, Math.round(bounds.height * ratio));
  context.setTransform(
    (bounds.width / BASE_WIDTH) * ratio,
    0,
    0,
    (bounds.height / BASE_HEIGHT) * ratio,
    0,
    0,
  );
}

function render(): void {
  if (canvas.width === 0 || canvas.height === 0) setCanvasScale();
  const range = Number(rangeInput.value);
  context.clearRect(0, 0, BASE_WIDTH, BASE_HEIGHT);
  context.fillStyle = "#111b19";
  context.fillRect(0, 0, BASE_WIDTH, BASE_HEIGHT);

  context.strokeStyle = "rgba(159, 186, 176, 0.055)";
  context.lineWidth = 1;
  for (let x = 40; x < BASE_WIDTH; x += 40) {
    context.beginPath();
    context.moveTo(x, 0);
    context.lineTo(x, BASE_HEIGHT);
    context.stroke();
  }
  for (let y = 40; y < BASE_HEIGHT; y += 40) {
    context.beginPath();
    context.moveTo(0, y);
    context.lineTo(BASE_WIDTH, y);
    context.stroke();
  }

  const source = nodes.find((node) => node.id === sourceId);
  if (source) {
    context.beginPath();
    context.arc(source.position.x, source.position.y, range, 0, Math.PI * 2);
    context.fillStyle = "rgba(95, 213, 171, 0.045)";
    context.fill();
    context.strokeStyle = "rgba(95, 213, 171, 0.24)";
    context.setLineDash([5, 8]);
    context.stroke();
    context.setLineDash([]);
  }

  for (let i = 0; i < nodes.length; i += 1) {
    const a = nodes[i];
    if (!a) continue;
    for (let j = i + 1; j < nodes.length; j += 1) {
      const b = nodes[j];
      if (!b || Math.hypot(a.position.x - b.position.x, a.position.y - b.position.y) > range) continue;
      context.beginPath();
      context.moveTo(a.position.x, a.position.y);
      context.lineTo(b.position.x, b.position.y);
      context.strokeStyle = "rgba(125, 159, 145, 0.25)";
      context.lineWidth = 1;
      context.setLineDash([3, 6]);
      context.stroke();
      context.setLineDash([]);
    }
  }

  for (const id of activeTransmissions) {
    const node = nodes.find((item) => item.id === id);
    if (!node) continue;
    context.beginPath();
    context.arc(node.position.x, node.position.y, range, 0, Math.PI * 2);
    context.fillStyle = "rgba(95, 213, 171, 0.09)";
    context.fill();
  }

  for (const node of nodes) {
    const isSource = node.id === sourceId;
    const isTx = activeTransmissions.has(node.id);
    const isCollision = collidedReceivers.has(node.id);
    const isReception = activeReceivers.has(node.id);
    const isReached = reachedIds.has(node.id);
    const fill = isCollision
      ? "#f3a84d"
      : isSource
        ? "#f27872"
        : isReception || isReached
          ? "#5fd5ab"
          : "#738d82";
    const radius = isTx ? 12 : isSource ? 9 : 7;

    context.beginPath();
    context.arc(node.position.x, node.position.y, radius + 6, 0, Math.PI * 2);
    context.fillStyle = isTx
      ? "rgba(95, 213, 171, 0.18)"
      : isSource
        ? "rgba(242, 120, 114, 0.14)"
        : "rgba(115, 141, 130, 0.10)";
    context.fill();
    context.beginPath();
    context.arc(node.position.x, node.position.y, radius, 0, Math.PI * 2);
    context.fillStyle = fill;
    context.fill();
    context.strokeStyle = "#15211e";
    context.lineWidth = 2;
    context.stroke();
    context.fillStyle = "#dce7df";
    context.font = "600 12px 'IBM Plex Mono', monospace";
    context.textAlign = "center";
    context.fillText(node.id, node.position.x, node.position.y - radius - 11);
    if (isCollision) {
      context.fillStyle = "#f4b65b";
      context.font = "700 13px sans-serif";
      context.fillText("!", node.position.x, node.position.y + 4);
    }
  }

  app.querySelector("#map-node-count")!.textContent = `${nodes.length} NODES`;
}

canvas.addEventListener("pointerdown", (event) => {
  const point = canvasPoint(event);
  const found = nodeAt(point);
  if (placingNode && !found) {
    placeNode(point);
    return;
  }
  if (!found) return;
  dragNodeId = found.id;
  canvas.setPointerCapture(event.pointerId);
  canvas.classList.add("is-dragging");
});
canvas.addEventListener("pointermove", (event) => {
  if (!dragNodeId) return;
  const point = canvasPoint(event);
  const node = nodes.find((item) => item.id === dragNodeId);
  if (!node) return;
  node.position.x = Math.min(BASE_WIDTH - 24, Math.max(24, point.x));
  node.position.y = Math.min(BASE_HEIGHT - 24, Math.max(24, point.y));
  if (hasRun) clearRun();
  render();
});
canvas.addEventListener("pointerup", () => {
  if (!dragNodeId) return;
  dragNodeId = undefined;
  canvas.classList.remove("is-dragging");
  renderNodeControls();
});
canvas.addEventListener("pointercancel", () => {
  dragNodeId = undefined;
  canvas.classList.remove("is-dragging");
});

app.querySelector<HTMLButtonElement>("#randomize-button")!.addEventListener("click", randomize);
addNodeButton.addEventListener("click", addNode);
app.querySelector<HTMLButtonElement>("#reset-run-button")!.addEventListener("click", clearRun);
runButton.addEventListener("click", runBroadcast);
rangeInput.addEventListener("input", () => {
  rangeValue.value = `${rangeInput.value} m`;
  rangeValue.textContent = `${rangeInput.value} m`;
  if (hasRun) clearRun();
  render();
});
sourceSelect.addEventListener("change", () => {
  sourceId = sourceSelect.value;
  clearRun();
  renderNodeControls();
});
for (const input of [seedInput, countInput, backoffInput]) {
  input.addEventListener("input", () => {
    input.setAttribute("aria-invalid", "false");
  });
}
window.addEventListener("resize", () => {
  setCanvasScale();
  render();
});
window.addEventListener("keydown", (event) => {
  const target = event.target;
  const isInteractiveTarget =
    target instanceof Element &&
    target.closest("button, input, select, textarea, a") !== null;
  if (event.key === "Enter" && !isInteractiveTarget) {
    runBroadcast();
  }
  if (event.key === "Escape") {
    if (placingNode) setPlacementMode(false);
    clearRun();
  }
});

nodes = generateNodes(18, 42);
sourceId = nodes[0]?.id ?? "";
renderNodeControls();
setCanvasScale();
render();
