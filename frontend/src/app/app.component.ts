import { Component } from '@angular/core';
import {SimulationService} from './services/simulation.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'frontend';

  constructor(private simulation: SimulationService) {
  }
}
