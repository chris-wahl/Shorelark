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

CanvasRenderingContext2D.prototype.draw_triangle = function(x, y, size, rotation) {
    this.beginPath();

    const multiplier = 1.5;
    const origin_x = x + Math.cos(rotation) * size * multiplier;
    const origin_y = y + Math.sin(rotation) * size * multiplier;

    this.moveTo(origin_x, origin_y);
    this.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size);
    this.lineTo(
        x + Math.cos(rotation - 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation - 2.0 / 3.0 * Math.PI) * size,
    );
    this.lineTo(origin_x, origin_y);

    // this.fillStyle = 'rgb(0, 0, 0)';
    // this.fill();
    this.stroke();

}
for (const animal of simulation.world().animals) {
    context.draw_triangle(animal.x * viewport_width, animal.y * viewport_height , 0.01 * viewport_width, animal.rotation);
}
