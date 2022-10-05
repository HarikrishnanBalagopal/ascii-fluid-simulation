async function main() {
    console.log('start');
    const res = await fetch('assets/wasm/test_fluid.wasm', { headers: { 'Accept': 'application/wasm' } });
    if (!res.ok) return console.error('failed to fetch the web-assembly module. status:', res.statusText);
    const moduleBytes = await res.arrayBuffer();
    const importObject = {};
    const module = await WebAssembly.instantiate(moduleBytes, importObject);
    console.log('module', module);
    // module.instance.exports.main();
    console.log('done');
}

main().catch(console.error);
