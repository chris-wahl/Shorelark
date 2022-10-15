import {AfterViewInit, Component, DoCheck, OnInit, ViewChild} from '@angular/core';
import {SimulationService} from '../../services/simulation.service';

@Component({
  selector: 'app-sim-controls',
  templateUrl: './sim-controls.component.html',
  styleUrls: ['./sim-controls.component.scss']
})
export class SimControlsComponent implements OnInit {
  constructor(private simulation: SimulationService) {
  }

  ngOnInit(): void {
  }

  toggle_sim() {
    this.simulation.toggle();
  }
}
