import { Game, System } from "pagurus";

interface PixcilOptions {
  wasmPath: string;
  canvas: HTMLCanvasElement;
  canvasArea: HTMLDivElement;
}

class Pixcil {
  game: Game;
  system: System;

  constructor(game: Game, system: System, _options: PixcilOptions) {
    this.game = game;
    this.system = system;
  }

  static async load(options: PixcilOptions): Promise<Pixcil> {
    const canvas = options.canvas;
    const canvasArea = options.canvasArea;
    const game = await Game.load(options.wasmPath);
    const system = await System.create(game.memory, { canvas });

    const resizeCanvas = () => {
      canvas.height = canvasArea.clientHeight;
      canvas.width = canvasArea.clientWidth;
      system.notifyRedrawNeeded();
    };
    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);

    game.initialize(system);

    return new Pixcil(game, system, options);
  }

  async run(): Promise<void> {}
}

export { Pixcil, PixcilOptions };
