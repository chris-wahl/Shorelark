import {Injectable} from '@angular/core';
import * as sim from "lib-simulation-wasm";
import {Bird} from './models/Bird';
import {assertCompatibleAngularVersion} from '@angular-devkit/build-angular/src/utils/version';

@Injectable({
  providedIn: 'root'
})
export class SimulationService {
  readonly MULTIPLIER = 1.5;
  private simulation: sim.Simulation;
  private is_running = true;

  constructor() {
    this.simulation = new sim.Simulation();
  }

  start() {
    this.is_running = true;
  }

  stop() {
    this.is_running = false;
  }

  toggle() {
    this.is_running = !this.is_running;
  }

  step() {
    if (this.is_running) this.simulation.step();
  }

  draw_birds(canvas: HTMLCanvasElement) {
    const context = canvas.getContext('2d');
    const size = canvas.width * 0.01;
    this.simulation.world().animals.forEach((bird: Bird) => this.draw_bird(
      context!,
      canvas.width,
      canvas.height,
      size,
      bird
    ))
  }

  private draw_bird(canvas: CanvasRenderingContext2D, width: number, height: number, size: number, bird: Bird) {
    canvas.beginPath();
    const x = bird.x * width;
    const y = bird.y * height;


    const origin_x = x + Math.cos(bird.rotation) * size * this.MULTIPLIER;
    const origin_y = y + Math.sin(bird.rotation) * size * this.MULTIPLIER;

    canvas.moveTo(origin_x, origin_y);
    canvas.lineTo(
      x + Math.cos(bird.rotation + 2.0 / 3.0 * Math.PI) * size,
      y + Math.sin(bird.rotation + 2.0 / 3.0 * Math.PI) * size,
    );
    canvas.lineTo(
      x + Math.cos(bird.rotation - 2.0 / 3.0 * Math.PI) * size,
      y + Math.sin(bird.rotation - 2.0 / 3.0 * Math.PI) * size,
    );
    canvas.lineTo(origin_x, origin_y);

    canvas.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
    canvas.fill();
  }
}
