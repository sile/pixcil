import { Game, System } from "pagurus";

async function installServiceWorker(serviceWorkerPath: string) {
  if ("serviceWorker" in navigator) {
    await navigator.serviceWorker.register(serviceWorkerPath);
  }
}

interface Parent {
  postMessage(message: object): void;
}

interface Options {
  wasmPath: string;
  canvas: HTMLCanvasElement;
  canvasArea: HTMLDivElement;
  parent: Parent;
  disableSaveWorkspaceButton?: boolean;
  enableDirtyNotification?: boolean;
  workspacePath?: string;
}

type Message = {
  data: MessageData;
};

type MessageData =
  | { type: "setWorkspace"; requestId: number; body: Uint8Array }
  | { type: "getWorkspace"; requestId: number }
  | { type: "notifyInputSize"; requestId: number; body: { id: number; size: string } }
  | { type: "notifyInputNumber"; requestId: number; body: { id: number; number: string } };

class App {
  private game: Game;
  private system: System;
  private parent: Parent;
  private gameStateVersion = BigInt(0);
  private dirtyNotificationEnabled: boolean;
  private isDirty = false;
  private dirtyNotificationTimeout?: number;
  private idle = false;

  constructor(game: Game, system: System, options: Options) {
    this.game = game;
    this.system = system;
    this.parent = options.parent;
    this.dirtyNotificationEnabled = options.enableDirtyNotification === true;

    window.addEventListener("message", (msg: Message) => this.handleMessage(msg));

    if (options.disableSaveWorkspaceButton) {
      game.command(system, "disableSaveWorkspaceButton", new Uint8Array());
    }

    options.canvas.onpointerdown = (e) => this.handlePointerEvent(e);
    options.canvas.onpointermove = (e) => this.handlePointerEvent(e);
    options.canvas.onpointerup = (e) => this.handlePointerEvent(e);
    options.canvas.onpointercancel = (e) => this.handlePointerEvent(e);
    options.canvas.onpointerout = (e) => this.handlePointerEvent(e);
    options.canvas.onpointerover = (e) => this.handlePointerEvent(e);

    this.parent.postMessage({ type: "ready" });
  }

  private handleMessage(msg: Message): void {
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
    } catch (error) {
      console.warn(error);
      this.parent.postMessage({ type: "errorResponse", requestId: msg.data.requestId, error });
    }
  }

  private handlePointerEvent(event: PointerEvent): void {
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

  static async load(options: Options): Promise<App> {
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

  async run(): Promise<void> {
    for (;;) {
      if (!(await this.runOnce())) {
        break;
      }
    }
  }

  private stateVersion(): bigint {
    return new DataView(this.game.query(this.system, "stateVersion").buffer).getBigInt64(0, false);
  }

  private handleDirtyState(): void {
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

  private notifyDirtyIfNeed(): void {
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

  private async runOnce(): Promise<boolean> {
    const event = await this.system.nextEvent();

    if (!this.game.handleEvent(this.system, event)) {
      return false;
    }

    if (this.dirtyNotificationEnabled) {
      this.handleDirtyState();
    }

      type RequestJson = "saveWorkspace"
          | "loadWorkspace"
          | "importImage"
          | { inputNumber: { id: number } }
          | { inputSize: { id: number } }
          | "vibrate";

    const requestBytes = this.game.query(this.system, "nextIoRequest");
    if (requestBytes.length > 0) {
      const requestJson = JSON.parse(new TextDecoder("utf-8").decode(requestBytes)) as RequestJson;
      switch (requestJson) {
        case "saveWorkspace":
          this.saveWorkspace();
          break;
        case "loadWorkspace":
          this.loadWorkspace();
          break;
        case "importImage":
          this.importImage();
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
          } else if ("inputSize" in requestJson) {
            const inputId = requestJson.inputSize.id;
            this.parent.postMessage({ type: "inputSize", inputId });
          }
      }
    }
    return true;
  }

  private saveWorkspace() {
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

  private importImage() {
    const input = document.createElement("input");
    input.setAttribute("type", "file");
    input.setAttribute("accept", "image/png");
    input.onchange = async () => {
      const files = input.files;
      if (files === null || files.length === 0) {
        return;
      }

      const file = files[0];

      const data = new Uint8Array(await file.arrayBuffer());
      try {
        this.game.command(this.system, "importImage", data);
      } catch (e) {
        console.warn(e);
        alert("Failed to load PNG file");
      }
    };
    input.click();
  }

  private loadWorkspace() {
    const input = document.createElement("input");
    input.setAttribute("type", "file");
    input.setAttribute("accept", "image/png");
    input.onchange = async () => {
      const files = input.files;
      if (files === null || files.length === 0) {
        return;
      }

      const file = files[0];

      const data = new Uint8Array(await file.arrayBuffer());
      try {
        this.game.command(this.system, "loadWorkspace", data);
      } catch (e) {
        console.warn(e);
        alert("Failed to load workspace file");
      }
    };
    input.click();
  }

  private generateWorkspaceName() {
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

export { App, Options, installServiceWorker, Parent };
