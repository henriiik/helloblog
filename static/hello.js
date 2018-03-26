fetch("/static/hellowasm.gc.wasm")
  .then(r => r.arrayBuffer())
  .then(r => WebAssembly.instantiate(r))
  .then(wasm_module => {
    const element = document.createElement("small");
    element.appendChild(
      document.createTextNode(
        `Hello from WASM! 2 + 1 = ${wasm_module.instance.exports.add_one(2)}`
      )
    );
    document.body.appendChild(element);
  });
