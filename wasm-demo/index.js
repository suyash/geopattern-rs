import * as wasm from "./pkg";

const container = document.querySelector("#container");
const input = document.querySelector("#input");

const image = wasm.generate_base64_svg_string("");
container.style.background = `url("data:image/svg+xml;base64,${image}")`;

input.addEventListener("keyup", () => {
    const image = wasm.generate_base64_svg_string(input.value);
    container.style.background = `url("data:image/svg+xml;base64,${image}")`;
});
