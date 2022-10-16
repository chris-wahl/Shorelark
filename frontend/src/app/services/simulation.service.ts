import {Injectable} from '@angular/core';
import * as sim from "lib-simulation-wasm";
import {Bird} from './models/Bird';
import {Food} from './models/Food';
import {BehaviorSubject} from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class SimulationService {
  readonly MULTIPLIER = 1.5;
  private simulation: sim.Simulation;
  private is_running = true;
  public readonly training_subject = new BehaviorSubject<boolean>(false);
  public readonly stats_subject = new BehaviorSubject<any>(null);

  constructor() {
    this.simulation = new sim.Simulation();
  }

  running() {
    return this.is_running;
  }

  toggle() {
    this.is_running = !this.is_running;
  }

  step() {
    if (this.is_running) this.simulation.step();
  }

  train(steps_to_train: number) {
    this.training_subject.next(true);
    this.run_training(steps_to_train).then(result => {
      console.log(result);
      this.stats_subject.next(result);
      this.training_subject.next(false);
    });
  }

  async run_training(steps_to_train: number) {
    return this.simulation.train(steps_to_train);
  }

  render_to_canvas(canvas: HTMLCanvasElement) {
    this.draw_food(canvas);
    this.draw_birds(canvas);
  }

  private draw_food(canvas: HTMLCanvasElement) {
    const context = canvas.getContext('2d')!;
    const viewport_width = canvas.width;
    const viewport_height = canvas.height;
    const radius = (0.01 / 2.0) * canvas.width;
    const end_angle = 2.0 * Math.PI;
    this.simulation.world().foods.forEach((food: Food) => {
      context.beginPath();
      context.arc(food.x * viewport_width, food.y * viewport_height, radius, 0, end_angle);
      context.fillStyle = 'rgb(0, 255, 128)';
      context.fill();
    });
  }

  private draw_birds(canvas: HTMLCanvasElement) {
    const context = canvas.getContext('2d')!;
    const size = canvas.width * 0.01;
    this.simulation.world().animals.forEach((bird: Bird) => {
      context.beginPath();
      const x = bird.x * canvas.width;
      const y = bird.y * canvas.height;


      const origin_x = x + Math.cos(bird.rotation) * size * this.MULTIPLIER;
      const origin_y = y + Math.sin(bird.rotation) * size * this.MULTIPLIER;

      context.moveTo(origin_x, origin_y);
      context.lineTo(
        x + Math.cos(bird.rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(bird.rotation + 2.0 / 3.0 * Math.PI) * size,
      );
      context.lineTo(
        x + Math.cos(bird.rotation - 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(bird.rotation - 2.0 / 3.0 * Math.PI) * size,
      );
      context.lineTo(origin_x, origin_y);

      context.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
      context.fill();
    });
  }
}
