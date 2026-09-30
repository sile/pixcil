(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define(['exports'], factory) :
  (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global.Pixcil = {}));
})(this, (function (exports) { 'use strict';

  const AUDIO_WORKLET_PROCESSOR_CODE = `
class PagurusAudioWorkletProcessor extends AudioWorkletProcessor {
  constructor() {
    super();
    this.inputBuffer = [];
    this.offset = 0;
    this.port.onmessage = (e) => {
      this.inputBuffer.push(e.data);
    };
  }

  process(inputs, outputs, parameters) {
    const outputChannel = outputs[0][0];
    for (let i = 0; i < outputChannel.length; i++) {
      const audioData = this.inputBuffer[0];
      if (audioData === undefined) {
        outputChannel[i] = 0;
      } else {
        outputChannel[i] = audioData[this.offset];
        this.offset++;
        if (this.offset == audioData.length) {
          this.inputBuffer.shift();
          this.offset = 0;
        }
      }
    }
    return true;
  }
}

registerProcessor("pagurus-audio-worklet-processor", PagurusAudioWorkletProcessor);
`;
  const AUDIO_WORKLET_PROCESSOR_NAME = "pagurus-audio-worklet-processor";
  function toPagurusKey(key) {
      switch (key) {
          case "ArrowUp":
              return "up";
          case "ArrowDown":
              return "down";
          case "ArrowLeft":
              return "left";
          case "ArrowRight":
              return "right";
          case "Enter":
              return "return";
          case "Backspace":
              return "backspace";
          case "Delete":
              return "delete";
          case "Tab":
              return "tab";
          case "Escape":
              return "esc";
          default:
              if (isSpecialKey(key)) {
                  return;
              }
              return { char: key };
      }
  }
  function isSpecialKey(key) {
      return /^[a-zA-Z][a-zA-Z]+$/.test(key);
  }
  class System {
      wasmMemory;
      canvas;
      audioContext;
      audioInputNode;
      audioSampleRate;
      startTime;
      eventQueue;
      resolveNextEvent;
      propagateControlKey;
      static create(wasmMemory, options = {}) {
          return new System(wasmMemory, options.canvas, options);
      }
      constructor(wasmMemory, canvas, options) {
          this.wasmMemory = wasmMemory;
          this.propagateControlKey = !(options.propagateControlKey === false);
          let canvasSize = { width: 0, height: 0 };
          this.canvas = canvas;
          if (this.canvas !== undefined) {
              canvasSize = { width: this.canvas.width, height: this.canvas.height };
              this.canvas.style.width = `${canvasSize.width}px`;
              this.canvas.style.height = `${canvasSize.height}px`;
          }
          this.startTime = performance.now();
          if (this.canvas !== undefined) {
              if (!(options.disableKeyEvents === true)) {
                  document.addEventListener("keyup", (event) => {
                      this.handleKeyup(event);
                      this.preventKeyEventDefaultIfNeed(event);
                  });
                  document.addEventListener("keydown", (event) => {
                      this.preventKeyEventDefaultIfNeed(event);
                  });
              }
              if (!(options.disableMouseEvents === true)) {
                  this.canvas.addEventListener("mousemove", (event) => {
                      this.handleMousemove(event);
                  });
                  this.canvas.addEventListener("mousedown", (event) => {
                      this.handleMousedown(event);
                  });
                  this.canvas.addEventListener("mouseup", (event) => {
                      this.handleMouseup(event);
                  });
              }
              if (!(options.disableTouchEvents === true)) {
                  this.canvas.addEventListener("touchmove", (event) => {
                      this.handleTouchmove(event);
                      event.stopPropagation();
                      event.preventDefault();
                  });
                  this.canvas.addEventListener("touchstart", (event) => {
                      this.handleTouchstart(event);
                      event.stopPropagation();
                      event.preventDefault();
                  });
                  this.canvas.addEventListener("touchend", (event) => {
                      this.handleTouchend(event);
                      event.stopPropagation();
                      event.preventDefault();
                  });
              }
          }
          const initialEvent = { windowResized: canvasSize };
          this.eventQueue = [initialEvent];
      }
      nextEvent() {
          const event = this.eventQueue.shift();
          if (event !== undefined) {
              return Promise.resolve(event);
          }
          else {
              return new Promise((resolve) => {
                  this.resolveNextEvent = resolve;
              });
          }
      }
      preventKeyEventDefaultIfNeed(event) {
          if (this.propagateControlKey) {
              if (event.ctrlKey || event.key == "Control") {
                  return;
              }
          }
          event.stopPropagation();
          event.preventDefault();
      }
      handleKeyup(event) {
          const key = toPagurusKey(event.key);
          if (key !== undefined) {
              const ctrl = event.ctrlKey;
              const alt = event.altKey;
              if (key == "tab" && event.shiftKey) {
                  this.enqueueEvent({ key: { key: "backTab", ctrl, alt } });
              }
              else {
                  this.enqueueEvent({ key: { key, ctrl, alt } });
              }
          }
      }
      touchPosition(touch) {
          if (this.canvas === undefined) {
              throw new Error("bug");
          }
          const rect = this.canvas.getBoundingClientRect();
          return { x: Math.round(touch.clientX - rect.left), y: Math.round(touch.clientY - rect.top) };
      }
      handleTouchmove(event) {
          const touches = event.changedTouches;
          for (let i = 0; i < touches.length; i++) {
              const touch = touches[i];
              const position = this.touchPosition(touch);
              this.enqueueEvent({ mouse: { move: { position } } });
              break;
          }
      }
      handleTouchstart(event) {
          const touches = event.changedTouches;
          for (let i = 0; i < touches.length; i++) {
              const touch = touches[i];
              const position = this.touchPosition(touch);
              this.enqueueEvent({ mouse: { down: { position } } });
              break;
          }
      }
      handleTouchend(event) {
          const touches = event.changedTouches;
          for (let i = 0; i < touches.length; i++) {
              const touch = touches[i];
              const position = this.touchPosition(touch);
              this.enqueueEvent({ mouse: { up: { position } } });
              break;
          }
      }
      handleMousemove(event) {
          const x = event.offsetX;
          const y = event.offsetY;
          this.enqueueEvent({ mouse: { move: { position: { x, y } } } });
      }
      handleMousedown(event) {
          const x = event.offsetX;
          const y = event.offsetY;
          if (event.button === 0) {
              this.enqueueEvent({ mouse: { down: { position: { x, y } } } });
          }
      }
      handleMouseup(event) {
          const x = event.offsetX;
          const y = event.offsetY;
          if (event.button === 0) {
              this.enqueueEvent({ mouse: { up: { position: { x, y } } } });
          }
      }
      enqueueEvent(event) {
          if (this.resolveNextEvent !== undefined) {
              this.resolveNextEvent(event);
              this.resolveNextEvent = undefined;
          }
          else {
              this.eventQueue.push(event);
          }
      }
      requestRedraw() {
          if (this.canvas === undefined) {
              return;
          }
          const canvasSize = { width: this.canvas.width, height: this.canvas.height };
          this.canvas.style.width = `${canvasSize.width}px`;
          this.canvas.style.height = `${canvasSize.height}px`;
          this.enqueueEvent({ windowResized: canvasSize });
      }
      videoInit(width, _height, pixelFormatPtr, stridePtr) {
          new DataView(this.wasmMemory.buffer).setUint8(pixelFormatPtr, 1); // 1=RGB32
          new DataView(this.wasmMemory.buffer).setUint32(stridePtr, width, true);
      }
      videoDraw(videoFrameOffset, videoFrameLen, width, stride, format) {
          if (this.canvas === undefined) {
              return;
          }
          if (format != 1) {
              throw new Error(`expected RGB32(3) format, but got ${format}`);
          }
          if (width != stride) {
              throw new Error(`width ${width} differs from stride ${stride}`);
          }
          if (width === 0 || videoFrameLen === 0) {
              return;
          }
          const canvasCtx = this.canvas.getContext("2d");
          if (!canvasCtx) {
              throw Error("failed to get canvas 2D context");
          }
          const height = videoFrameLen / 4 / width;
          const videoFrame = new Uint8ClampedArray(this.wasmMemory.buffer, videoFrameOffset, videoFrameLen);
          if (width != this.canvas.width || height != this.canvas.height) {
              const xScale = width / this.canvas.width;
              const yScale = height / this.canvas.height;
              this.canvas.width = width;
              this.canvas.height = height;
              canvasCtx.scale(xScale, yScale);
          }
          const image = new ImageData(videoFrame, width, height);
          canvasCtx.putImageData(image, 0, 0);
      }
      audioInit(sampleRate, _dataSamples, sampleFormatPtr) {
          this.audioSampleRate = sampleRate;
          const littleEndian = (function () {
              const buffer = new ArrayBuffer(2);
              new DataView(buffer).setInt16(0, 256, true);
              return new Int16Array(buffer)[0] === 256;
          })();
          if (littleEndian) {
              new DataView(this.wasmMemory.buffer).setUint8(sampleFormatPtr, 3); // 3=F32Le
          }
          else {
              new DataView(this.wasmMemory.buffer).setUint8(sampleFormatPtr, 2); // 2=F32Be
          }
      }
      audioEnqueue(audioDataOffset, audioDataLen) {
          if (this.audioSampleRate === undefined) {
              console.warn("audioInit() has not been called yet");
              return;
          }
          const data = new Float32Array(this.wasmMemory.buffer, audioDataOffset, audioDataLen / 4).slice();
          if (this.audioContext === undefined) {
              const blob = new Blob([AUDIO_WORKLET_PROCESSOR_CODE], { type: "application/javascript" });
              const audioContext = new AudioContext({ sampleRate: this.audioSampleRate });
              this.audioContext = audioContext;
              this.audioContext.audioWorklet
                  .addModule(URL.createObjectURL(blob))
                  .then(() => {
                  this.audioInputNode = new AudioWorkletNode(audioContext, AUDIO_WORKLET_PROCESSOR_NAME);
                  this.audioInputNode.connect(audioContext.destination);
                  this.audioInputNode.port.postMessage(data, [data.buffer]);
              })
                  .catch((error) => {
                  throw error;
              });
          }
          else if (this.audioInputNode !== undefined) {
              this.audioInputNode.port.postMessage(data, [data.buffer]);
          }
      }
      consoleLog(messageOffset, messageLen) {
          const message = this.getWasmString(messageOffset, messageLen);
          console.log(message);
      }
      clockGameTime() {
          return (performance.now() - this.startTime) / 1000;
      }
      clockUnixTime() {
          return new Date().getTime() / 1000;
      }
      clockSetTimeout(tag, timeout) {
          setTimeout(() => {
              this.enqueueEvent({ timeout: tag });
          }, timeout * 1000);
      }
      getWasmString(offset, len) {
          const buffer = new Uint8Array(this.wasmMemory.buffer, offset, len);
          return new TextDecoder("utf-8").decode(buffer);
      }
  }
  class Game {
      wasmInstance;
      gameInstance;
      systemRef;
      memory;
      constructor(wasmInstance, systemRef) {
          this.wasmInstance = wasmInstance;
          this.gameInstance = wasmInstance.exports.gameNew();
          this.memory = wasmInstance.exports.memory;
          this.systemRef = systemRef;
      }
      static async load(gameWasmPath) {
          const systemRef = new SystemRef();
          const importObject = {
              env: {
                  consoleLog(messageOffset, messageLen) {
                      systemRef.getSystem().consoleLog(messageOffset, messageLen);
                  },
                  systemVideoInit(width, height, pixelFormatPtr, stridePtr) {
                      systemRef.getSystem().videoInit(width, height, pixelFormatPtr, stridePtr);
                  },
                  systemVideoDraw(videoFrameOffset, videoFrameLen, width, stride, format) {
                      systemRef.getSystem().videoDraw(videoFrameOffset, videoFrameLen, width, stride, format);
                  },
                  systemAudioInit(sampleRate, dataSamples, sampleFormatPtr) {
                      systemRef.getSystem().audioInit(sampleRate, dataSamples, sampleFormatPtr);
                  },
                  systemAudioEnqueue(dataOffset, dataLen) {
                      systemRef.getSystem().audioEnqueue(dataOffset, dataLen);
                  },
                  systemClockGameTime() {
                      return systemRef.getSystem().clockGameTime();
                  },
                  systemClockUnixTime() {
                      return systemRef.getSystem().clockUnixTime();
                  },
                  systemClockSetTimeout(tag, timeout) {
                      systemRef.getSystem().clockSetTimeout(tag, timeout);
                  },
              },
          };
          const results = await WebAssembly.instantiateStreaming(fetch(gameWasmPath), importObject);
          const wasmInstance = results.instance;
          return new Game(wasmInstance, systemRef);
      }
      initialize(system) {
          this.systemRef.setSystem(system);
          try {
              const error = this.wasmInstance.exports.gameInitialize(this.gameInstance);
              if (error !== 0) {
                  throw new Error(this.getWasmString(error));
              }
          }
          finally {
              this.systemRef.clearSystem();
          }
      }
      handleEvent(system, event) {
          this.systemRef.setSystem(system);
          try {
              const eventBytesPtr = this.createWasmBytes(new TextEncoder().encode(JSON.stringify(event)));
              const result = this.wasmInstance.exports.gameHandleEvent(this.gameInstance, eventBytesPtr);
              if (result === 0) {
                  return true;
              }
              const error = this.getWasmString(result);
              if (JSON.parse(error) === null) {
                  return false;
              }
              else {
                  throw new Error(error);
              }
          }
          finally {
              this.systemRef.clearSystem();
          }
      }
      query(system, name) {
          this.systemRef.setSystem(system);
          try {
              const nameBytesPtr = this.createWasmBytes(new TextEncoder().encode(name));
              const result = this.wasmInstance.exports.gameQuery(this.gameInstance, nameBytesPtr);
              const bytes = this.getWasmBytes(result);
              if (bytes[bytes.length - 1] === 0) {
                  return bytes.subarray(0, bytes.length - 1);
              }
              else {
                  const error = new TextDecoder("utf-8").decode(bytes.subarray(0, bytes.length - 1));
                  throw new Error(error);
              }
          }
          finally {
              this.systemRef.clearSystem();
          }
      }
      command(system, name, data) {
          this.systemRef.setSystem(system);
          try {
              const nameBytesPtr = this.createWasmBytes(new TextEncoder().encode(name));
              const dataBytesPtr = this.createWasmBytes(data);
              const result = this.wasmInstance.exports.gameCommand(this.gameInstance, nameBytesPtr, dataBytesPtr);
              if (result !== 0) {
                  const error = this.getWasmString(result);
                  throw new Error(error);
              }
          }
          finally {
              this.systemRef.clearSystem();
          }
      }
      createWasmBytes(bytes) {
          const wasmBytesPtr = this.wasmInstance.exports.memoryAllocateBytes(bytes.length);
          const offset = this.wasmInstance.exports.memoryBytesOffset(wasmBytesPtr);
          const len = this.wasmInstance.exports.memoryBytesLen(wasmBytesPtr);
          new Uint8Array(this.memory.buffer, offset, len).set(bytes);
          return wasmBytesPtr;
      }
      getWasmString(wasmBytesPtr) {
          try {
              const offset = this.wasmInstance.exports.memoryBytesOffset(wasmBytesPtr);
              const len = this.wasmInstance.exports.memoryBytesLen(wasmBytesPtr);
              const bytes = new Uint8Array(this.memory.buffer, offset, len);
              return new TextDecoder("utf-8").decode(bytes);
          }
          finally {
              this.wasmInstance.exports.memoryFreeBytes(wasmBytesPtr);
          }
      }
      getWasmBytes(wasmBytesPtr) {
          try {
              const offset = this.wasmInstance.exports.memoryBytesOffset(wasmBytesPtr);
              const len = this.wasmInstance.exports.memoryBytesLen(wasmBytesPtr);
              return new Uint8Array(this.memory.buffer, offset, len).slice();
          }
          finally {
              this.wasmInstance.exports.memoryFreeBytes(wasmBytesPtr);
          }
      }
  }
  class SystemRef {
      system;
      getSystem() {
          if (this.system === undefined) {
              throw Error("SystemRef.system is undefined");
          }
          return this.system;
      }
      setSystem(system) {
          this.system = system;
      }
      clearSystem() {
          this.system = undefined;
      }
  }

  async function installServiceWorker(serviceWorkerPath) {
      if ("serviceWorker" in navigator) {
          await navigator.serviceWorker.register(serviceWorkerPath);
      }
  }
  class App {
      game;
      system;
      parent;
      gameStateVersion = BigInt(0);
      dirtyNotificationEnabled;
      isDirty = false;
      dirtyNotificationTimeout;
      idle = false;
      constructor(game, system, options) {
          this.game = game;
          this.system = system;
          this.parent = options.parent;
          this.dirtyNotificationEnabled = options.enableDirtyNotification === true;
          window.addEventListener("message", (msg) => this.handleMessage(msg));
          if (options.disableSaveWorkspaceButton) {
              game.command(system, "disableSaveWorkspaceButton", new Uint8Array());
          }
          options.canvas.onpointerdown = (e) => this.handlePointerEvent(e);
          options.canvas.onpointermove = (e) => this.handlePointerEvent(e);
          options.canvas.onpointerup = (e) => this.handlePointerEvent(e);
          options.canvas.onpointercancel = (e) => this.handlePointerEvent(e);
          options.canvas.onpointerout = (e) => this.handlePointerEvent(e);
          options.canvas.onpointerleave = (e) => this.handlePointerEvent(e);
          this.parent.postMessage({ type: "ready" });
      }
      handleMessage(msg) {
          try {
              switch (msg.data.type) {
                  case "setWorkspace":
                      this.game.command(this.system, "loadWorkspace", msg.data.body);
                      this.isDirty = false;
                      this.gameStateVersion = this.stateVersion();
                      break;
                  case "getWorkspace":
                      {
                          const data = this.game.query(this.system, "workspacePng");
                          this.parent.postMessage({ type: "response", requestId: msg.data.requestId, body: data });
                          this.isDirty = false;
                          if (this.dirtyNotificationTimeout !== undefined) {
                              clearTimeout(this.dirtyNotificationTimeout);
                              this.dirtyNotificationTimeout = undefined;
                          }
                          this.gameStateVersion = this.stateVersion();
                      }
                      break;
                  case "notifyInputNumber":
                      {
                          const inputJsonBytes = new TextEncoder().encode(JSON.stringify(msg.data.body));
                          this.game.command(this.system, "notifyInputNumber", inputJsonBytes);
                      }
                      break;
                  case "notifyInputSize":
                      {
                          const inputJsonBytes = new TextEncoder().encode(JSON.stringify(msg.data.body));
                          this.game.command(this.system, "notifyInputSize", inputJsonBytes);
                      }
                      break;
              }
          }
          catch (error) {
              console.warn(error);
              this.parent.postMessage({ type: "errorResponse", requestId: msg.data.requestId, error });
          }
      }
      handlePointerEvent(event) {
          const data = {
              eventType: event.type,
              x: Math.round(event.offsetX),
              y: Math.round(event.offsetY),
              pointerId: event.pointerId,
              pointerType: event.pointerType,
              isPrimary: event.isPrimary,
          };
          const jsonBytes = new TextEncoder().encode(JSON.stringify(data));
          this.game.command(this.system, "handlePointerEvent", jsonBytes);
      }
      static async load(options) {
          const canvas = options.canvas;
          const canvasCtx = canvas.getContext("2d");
          if (canvasCtx != undefined) {
              canvasCtx.imageSmoothingEnabled = false;
          }
          const canvasArea = options.canvasArea;
          const game = await Game.load(options.wasmPath);
          const system = System.create(game.memory, { canvas, disableMouseEvents: true, disableTouchEvents: true });
          const onResizeCanvas = () => {
              canvas.height = canvasArea.clientHeight;
              canvas.width = canvasArea.clientWidth;
              system.requestRedraw();
          };
          onResizeCanvas();
          window.addEventListener("resize", onResizeCanvas);
          game.initialize(system);
          if (options.workspacePath) {
              const workspaceData = await fetch(options.workspacePath, { cache: "no-store" }).then((response) => response.arrayBuffer());
              game.command(system, "loadWorkspace", new Uint8Array(workspaceData));
          }
          return new App(game, system, options);
      }
      async run() {
          for (;;) {
              if (!(await this.runOnce())) {
                  break;
              }
          }
      }
      stateVersion() {
          return new DataView(this.game.query(this.system, "stateVersion").buffer).getBigInt64(0, false);
      }
      handleDirtyState() {
          this.idle = false;
          if (this.isDirty) {
              return;
          }
          const version = this.stateVersion();
          if (version === this.gameStateVersion) {
              return;
          }
          this.idle = true;
          this.notifyDirtyIfNeed();
      }
      notifyDirtyIfNeed() {
          if (this.idle) {
              const version = this.stateVersion();
              if (version !== this.gameStateVersion) {
                  this.gameStateVersion = version;
                  this.parent.postMessage({ type: "notifyDirty" });
              }
          }
          this.idle = true;
          this.dirtyNotificationTimeout = setTimeout(() => {
              this.notifyDirtyIfNeed();
          }, 1000);
      }
      async runOnce() {
          const event = await this.system.nextEvent();
          if (!this.game.handleEvent(this.system, event)) {
              return false;
          }
          if (this.dirtyNotificationEnabled) {
              this.handleDirtyState();
          }
          const requestBytes = this.game.query(this.system, "nextIoRequest");
          if (requestBytes.length > 0) {
              const requestJson = JSON.parse(new TextDecoder("utf-8").decode(requestBytes));
              switch (requestJson) {
                  case "saveWorkspace":
                      this.saveWorkspace();
                      break;
                  case "loadWorkspace":
                      this.loadWorkspace();
                      break;
                  case "vibrate":
                      if ("vibrate" in window.navigator) {
                          window.navigator.vibrate(50);
                      }
                      break;
                  default:
                      if ("inputNumber" in requestJson) {
                          const inputId = requestJson.inputNumber.id;
                          this.parent.postMessage({ type: "inputNumber", inputId });
                      }
                      else if ("inputSize" in requestJson) {
                          const inputId = requestJson.inputSize.id;
                          this.parent.postMessage({ type: "inputSize", inputId });
                      }
              }
          }
          return true;
      }
      saveWorkspace() {
          const name = prompt("Please input your workspace name", this.generateWorkspaceName());
          if (!name) {
              return;
          }
          const data = this.game.query(this.system, "workspacePng");
          const blob = new Blob([data], { type: "image/png" });
          const element = document.createElement("a");
          element.download = name + ".png";
          element.href = URL.createObjectURL(blob);
          element.click();
      }
      loadWorkspace() {
          const input = document.createElement("input");
          input.setAttribute("type", "file");
          input.setAttribute("accept", "image/png");
          // [NOTE] This is necessary to trigger the onchange event in Safari.
          document.body.appendChild(input);
          input.onchange = async () => {
              document.body.removeChild(input);
              const files = input.files;
              if (files === null || files.length === 0) {
                  return;
              }
              const file = files[0];
              const data = new Uint8Array(await file.arrayBuffer());
              try {
                  this.game.command(this.system, "loadWorkspace", data);
              }
              catch (e) {
                  console.warn(e);
                  alert("Failed to load workspace file");
              }
          };
          input.oncancel = async () => {
              document.body.removeChild(input);
          };
          input.click();
      }
      generateWorkspaceName() {
          const now = new Intl.DateTimeFormat([], {
              year: "numeric",
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit",
              second: "2-digit",
          })
              .format(new Date())
              .replaceAll(/[:/]/g, "")
              .replace(" ", "_");
          return `pixcil-${now}`;
      }
  }

  exports.App = App;
  exports.installServiceWorker = installServiceWorker;

}));
