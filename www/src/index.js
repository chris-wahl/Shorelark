import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
document.getElementById('train').onclick = function () {
    const result = simulation.train();
    set_result(result.min, result.max, result.avg);
};

const stats_min = document.getElementById('stats_min');
const stats_max = document.getElementById('stats_max');
const stats_avg = document.getElementById('stats_avg');

const viewport = document.getElementById('viewport');
const viewport_scale = window.devicePixelRatio || 1;
const [viewport_width, viewport_height] = [viewport.width, viewport.height];
viewport.width = viewport_width * viewport_scale;
viewport.height = viewport_height * viewport_scale;
viewport.style.width = viewport_width + 'px';
viewport.style.height = viewport_height + 'px';


const context = viewport.getContext('2d');
context.fillStyle = 'rgb(0, 0, 0)';

CanvasRenderingContext2D.prototype.draw_triangle = function (x, y, size, rotation) {
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

    this.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
    this.fill();
    // this.stroke();

}
CanvasRenderingContext2D.prototype.draw_circle = function (x, y, radius) {
    this.beginPath();
    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)'; // A nice green color
    this.fill();
}

function set_result(min, max, avg) {
    stats_min.textContent = min;
    stats_max.textContent = max;
    stats_avg.textContent = avg.toFixed(2);
}

function redraw() {
    context.clearRect(0, 0, viewport_width, viewport_height);
    const result = simulation.step();
    if (!!result) set_result(result.min, result.max, result.avg);

    const world = simulation.world();

    for (const food of world.foods) {
        context.draw_circle(food.x * viewport_width, food.y * viewport_height, (0.01 / 2.0) * viewport_width);
    }

    for (const animal of world.animals) {
        context.draw_triangle(animal.x * viewport_width, animal.y * viewport_height, 0.01 * viewport_width, animal.rotation);
    }

    requestAnimationFrame(redraw);
}

redraw();

