import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();

const viewport = document.getElementById('viewport');
const viewport_scale = window.devicePixelRatio || 1;
const [viewport_width, viewport_height] = [viewport.width, viewport.height];
viewport.width = viewport_width * viewport_scale;
viewport.height = viewport_height * viewport_scale;
viewport.style.width = viewport_width + 'px';
viewport.style.height = viewport_height + 'px';


const context = viewport.getContext('2d');
context.fillStyle = 'rgb(0, 0, 0)';

for (const animal of simulation.world().animals) {
    context.fillRect(
        animal.x * viewport_width,
        animal.y * viewport_height,
        15,
        15,
    );
}

