async function main() {
    const CONSOLE_WIDTH = 80;
    const CONSOLE_HEIGHT = 24;

    console.log('start');
    const res = await fetch('assets/wasm/test_fluid.wasm', { headers: { 'Accept': 'application/wasm' } });
    if (!res.ok) return console.error('failed to fetch the web-assembly module. status:', res.statusText);
    const moduleBytes = await res.arrayBuffer();
    const importObject = {};
    const module = await WebAssembly.instantiate(moduleBytes, importObject);
    console.log('module', module);
    module.instance.exports.initialize_global();
    const address = module.instance.exports.get_address_global();
    console.log('address', address);
    const mem = new Uint8Array(module.instance.exports.memory.buffer, address, CONSOLE_WIDTH * CONSOLE_HEIGHT + 1);
    console.log('mem', mem);
    let last_t = null;
    const fluid_output = document.querySelector('#fluid-output');
    const TIME_STEP = 1;
    const decoder = new TextDecoder();
    function draw(t) {
        requestAnimationFrame(draw);
        if (!last_t) last_t = t;
        if (t - last_t < TIME_STEP) return;
        last_t = t;
        module.instance.exports.step_global();
        fluid_output.textContent = decoder.decode(mem);
    }
    // requestAnimationFrame(draw);
    console.log('done');
}

main().catch(console.error);
