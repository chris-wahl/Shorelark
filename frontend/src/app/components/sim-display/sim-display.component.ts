import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {SimulationService} from '../../services/simulation.service';

@Component({
  selector: 'app-sim-display',
  templateUrl: './sim-display.component.html',
  styleUrls: ['./sim-display.component.scss']
})
export class SimDisplayComponent implements OnInit {
  canvas!: HTMLCanvasElement;
  canvas_2d_context!: CanvasRenderingContext2D;
  run_sim = true;

  constructor(private simulation: SimulationService) {
  }

  ngOnInit(): void {
  }

  ngAfterViewInit() {
    this.canvas = document.getElementById('sim_canvas') as HTMLCanvasElement;
    this.canvas_2d_context = this.canvas.getContext('2d') as CanvasRenderingContext2D;


    const viewport_scale = window.devicePixelRatio || 1;
    const [viewport_width, viewport_height] = [this.canvas.width, this.canvas.height];
    this.canvas.width = viewport_width * viewport_scale;
    this.canvas.height = viewport_height * viewport_scale;
    this.canvas.style.width = viewport_width + 'px';
    this.canvas.style.height = viewport_height + 'px';


    this.simulation.draw_birds(this.canvas);
    requestAnimationFrame(() => this.run_simulation());
  }

  run_simulation() {
    if (this.run_sim) {
      this.canvas_2d_context.clearRect(0, 0, this.canvas.width, this.canvas.height);
      this.simulation.step();
      this.simulation.draw_birds(this.canvas);
    }
    requestAnimationFrame(() => this.run_simulation());
  }
}
