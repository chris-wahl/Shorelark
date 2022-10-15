import { Injectable } from '@angular/core';
import * as sim from "lib-simulation-wasm";

@Injectable({
  providedIn: 'root'
})
export class SimulationService {
  private simulation: sim.Simulation;

  constructor() {
    this.simulation = new sim.Simulation();

  }
}
