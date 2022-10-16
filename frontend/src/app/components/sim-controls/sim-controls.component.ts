import {Component, OnInit} from '@angular/core';
import {SimulationService} from '../../services/simulation.service';
import {Observable, tap} from 'rxjs';

@Component({
  selector: 'app-sim-controls',
  templateUrl: './sim-controls.component.html',
  styleUrls: ['./sim-controls.component.scss']
})
export class SimControlsComponent implements OnInit {
  readonly MIN_TO_TRAIN = 1_000;
  readonly MAX_TO_TRAIN = 100_000;
  readonly TICK_INTERVAL = 10_000;
  steps_to_train: number | null = 2_000;

  is_training$: Observable<boolean>;


  constructor(private simulation: SimulationService) {
    this.is_training$ = this.simulation.training_subject.asObservable().pipe(tap(console.log));
  }

  ngOnInit() {
  }

  running() {
    return this.simulation.running();
  }

  toggle_sim() {
    this.simulation.toggle();
  }

  train() {
    const restart_running = this.simulation.running();
    if (restart_running) this.simulation.toggle();
    this.simulation.train(this.steps_to_train!);
  }
}
