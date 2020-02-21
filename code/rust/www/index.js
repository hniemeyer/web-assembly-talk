import * as wasm from "my-wasm-module";

const pre = document.getElementById("my-canvas");
const button = document.getElementById("next-number");
button.textContent = "Next fibonacci number please...";
const seq = wasm.FibSequence.new();
pre.textContent = " "


button.addEventListener("click", event => {
    pre.textContent = seq.gen_fibnumber();
  });