import {Component, OnInit} from '@angular/core';
import {SimulationService} from '../../services/simulation.service';

@Component({
  selector: 'app-sim-display',
  templateUrl: './sim-display.component.html',
  styleUrls: ['./sim-display.component.scss']
})
export class SimDisplayComponent implements OnInit {
  canvas!: HTMLCanvasElement;
  canvas_2d_context!: CanvasRenderingContext2D;

  constructor(private simulation: SimulationService) {
  }

  ngOnInit(): void {
  }

  ngAfterViewInit() {
    this.canvas = document.getElementById('sim_canvas') as HTMLCanvasElement;
    this.canvas_2d_context = this.canvas.getContext('2d') as CanvasRenderingContext2D;

    this.simulation.render_to_canvas(this.canvas);
    requestAnimationFrame(() => this.run_simulation());
  }

  run_simulation() {
    if (this.simulation.running()) {
      this.canvas_2d_context.clearRect(0, 0, this.canvas.width, this.canvas.height);
      this.simulation.step();
      this.simulation.render_to_canvas(this.canvas);
    }
    requestAnimationFrame(() => this.run_simulation());
  }
}
