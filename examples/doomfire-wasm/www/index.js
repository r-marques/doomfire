import { DoomFireWasm } from "doomfire-wasm";
import { memory } from "doomfire-wasm/doomfire_wasm_bg";

// initialize DOOM Fire state and get the width and height
const doomFire = DoomFireWasm.new();
const width = doomFire.width();
const height = doomFire.height();

// Initialize the canvas
const canvas = document.getElementById("doomfire-canvas");
canvas.width = width;
canvas.height = height;
    
const ctx = canvas.getContext("2d");

const renderLoop = () => {
    // call the draw method that will display and image in the canvas
    draw();
    // update the DOOM Fire state
    doomFire.update();

    requestAnimationFrame(renderLoop);

};

const draw = () => {
    // draw the current state into the internal buffer
    doomFire.draw();

    // get a pointer the internal buffer since we can directly access
    // webassembly memory from javascript. This way we don't need to always
    // copy the buffer into javascript heap.
    const bufferPtr = doomFire.get_buffer();
    // 4 bytes per pixel
    const buffer = new Uint8ClampedArray(memory.buffer, bufferPtr, width * height * 4);

    // create a new image
    const image = new ImageData(buffer, width, height);
    // display the image on the canvas
    ctx.putImageData(image, 0, 0);
};

renderLoop();
