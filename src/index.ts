const decoder = new TextDecoder();

async function run() {
    let { instance, module } = await WebAssembly.instantiateStreaming(fetch("example_bot.wasm"), {
        "env": {
            "bot_log": (ptr: number, len: number) => {
                let mem = <WebAssembly.Memory>instance.exports.memory;
                let str = decoder.decode(mem.buffer.slice(ptr, ptr + len));
                console.log(str);
            },
            "connect4_select_column": (column: number) => {
                console.log(`Select column: ${column}`);
            }
        }
    });

    let exports = <any>instance.exports;
    let state_ptr = exports.bot_init();
    exports.bot_update(state_ptr);
}

run();