const fs = require("fs");
const esbuild = require("esbuild");
const child_process = require("child_process");

esbuild.buildSync({
    bundle: true,
    minify: true,
    entryPoints: ["src/index.ts"],
    outfile: "dist/bundle.js"
});
fs.cpSync("static", "dist", { recursive: true });

child_process.execSync("cd example-bot && cargo build --release");
fs.cpSync("example-bot/target/wasm32-unknown-unknown/release/example_bot.wasm", "dist/example_bot.wasm");