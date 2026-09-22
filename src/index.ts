const decoder = new TextDecoder();

async function run() {
    let { instance, module } = await WebAssembly.instantiateStreaming(fetch("example_bot.wasm"), {
        "env": {
            "bot_log": (ptr: number, len: number) => {
                let mem = <WebAssembly.Memory>instance.exports.memory;
                let str = decoder.decode(mem.buffer.slice(ptr, ptr + len));
                console.log(str);
            }
        }
    });

    let exports = <any>instance.exports;
    exports.init();
}

run();