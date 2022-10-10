import * as sim from "lib-simulation-wasm";

const body = document.getElementsByTagName("body")[0];
const new_tag = document.createElement('p');
new_tag.textContent = "Who's that dog? " + sim.whos_that_dog() + "!";
body.append(new_tag);
