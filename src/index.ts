const decoder = new TextDecoder();

const cvs = document.querySelector<HTMLCanvasElement>("#cvs")!;
const ctx = cvs.getContext("2d")!;

let time = 0;

class Bot {
    index: number;
    instance?: WebAssembly.Instance;
    memory?: WebAssembly.Memory;
    state_ptr?: number;

    constructor(index: number) {
        this.index = index;
    }

    deocdeString(ptr: number, length: number): string {
        return decoder.decode(this.memory!.buffer.slice(ptr, ptr + length));
    }

    async init(): Promise<void> {
        let { instance } = await WebAssembly.instantiateStreaming(fetch("example_bot.wasm"), {
            "env": {
                "bot_log": (ptr: number, len: number) => console.log(this.deocdeString(ptr, len)),
                "connect4_select_column": (column: number) => {
                    selectedColumn = column;
                },
                "connect4_get_tile": (column: number, row: number) => {
                    switch (grid[row][column].state) {
                        case ChipState.Emtpy: return 0;
                        case ChipState.Black: return 1;
                        case ChipState.Red: return 2;
                    }
                }
            }
        });
        this.instance = instance;
        this.memory = <WebAssembly.Memory>instance.exports.memory;

        this.state_ptr = (<any>this.instance.exports).bot_init(this.index);
    }

    update(): void {
        (<any>this.instance!.exports).bot_update(this.state_ptr!);
    }
}

enum ChipState {
    Emtpy,
    Red,
    Black
}

type Chip = {
    state: ChipState,
    offset: number,
    velocity: number
}

const TILE_SIZE = 100;
const TILE_PAD = 8;
const GRID_COLS = 8;
const GRID_ROWS = 6;

let selectedColumn = 0;

let grid: Chip[][] = [];
for (let row = 0; row < GRID_ROWS; row++) {
    let row: Chip[] = [];
    for (let col = 0; col < GRID_COLS; col++) {
        row.push({ state: ChipState.Emtpy, offset: 0, velocity: 0 });
    }
    grid.push(row);
}

function dropChip(column: number, chip: ChipState): boolean {
    for (let row = 0; row < GRID_ROWS; row++) {
        if (grid[row][column].state != ChipState.Emtpy) {
            if (row == 0) {
                return false;
            } else {
                grid[row - 1][column] = {
                    state: chip,
                    offset: row * TILE_SIZE,
                    velocity: 0
                };
                return true;
            }
        }
    }
    grid[GRID_ROWS - 1][column] = {
        state: chip,
        offset: GRID_ROWS * TILE_SIZE,
        velocity: 0
    };
    return true;
}

function update(newTime: number) {
    requestAnimationFrame(t => update(t));
    let dt = newTime - time;
    time = newTime;

    ctx.rect(0, 0, cvs.width, cvs.height);
    ctx.fillStyle = "blue";
    ctx.fill();

    for (let row = 0; row < GRID_ROWS; row++) {
        for (let col = 0; col < GRID_COLS; col++) {
            let tile = grid[row][col];
            if (tile.state == ChipState.Red) {
                ctx.fillStyle = "red";
            } else if (tile.state == ChipState.Black) {
                ctx.fillStyle = "black";
            } else {
                continue;
            }
            if (tile.offset > 0) {
                tile.offset = Math.max(0, tile.offset - (dt * tile.velocity));
                tile.velocity += 0.008 * dt;
            }
            ctx.beginPath();
            ctx.arc((col + 0.5) * TILE_SIZE, (row + 0.5) * TILE_SIZE - tile.offset, TILE_SIZE / 2, 0, Math.PI * 2);
            ctx.fill();
        }
    }
}

function updateSize() {
    cvs.width = cvs.clientWidth;
    cvs.height = cvs.clientHeight;
}

function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    let blackPlayer = new Bot(0);
    await blackPlayer.init();

    let redPlayer = new Bot(1);
    await redPlayer.init();

    while (true) {
        redPlayer.update();
        dropChip(selectedColumn, ChipState.Red);
        await sleep(1000);

        blackPlayer.update();
        dropChip(selectedColumn, ChipState.Black);
        await sleep(1000);
    }
}

window.addEventListener("resize", () => updateSize());

run();
updateSize();
update(0);