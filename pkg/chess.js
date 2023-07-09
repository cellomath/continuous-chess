
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
*/
export const PieceType = Object.freeze({ Pawn:0,"0":"Pawn",Knight:1,"1":"Knight",Bishop:2,"2":"Bishop",Rook:3,"3":"Rook",Queen:4,"4":"Queen",King:5,"5":"King", });
/**
*/
export const PieceTeam = Object.freeze({ White:0,"0":"White",Black:1,"1":"Black", });
/**
*/
export class Board {

    static __wrap(ptr) {
        const obj = Object.create(Board.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.board_default_setup();
        return Board.__wrap(ret);
    }
    /**
    * @param {any} from
    * @param {any} to
    * @returns {boolean}
    */
    movePiece(from, to) {
        try {
            const ret = wasm.board_movePiece(this.ptr, addBorrowedObject(from), addBorrowedObject(to));
            return ret !== 0;
        } finally {
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {any} position
    * @returns {Piece | undefined}
    */
    get_piece_at(position) {
        try {
            const ret = wasm.board_get_piece_at(this.ptr, addBorrowedObject(position));
            return ret === 0 ? undefined : Piece.__wrap(ret);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    */
    run_damage_calculation() {
        wasm.board_run_damage_calculation(this.ptr);
    }
}
/**
*/
export class Coordinate {

    static __wrap(ptr) {
        const obj = Object.create(Coordinate.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_coordinate_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        const ret = wasm.__wbg_get_coordinate_x(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_coordinate_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        const ret = wasm.__wbg_get_coordinate_y(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_coordinate_y(this.ptr, arg0);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    constructor(x, y) {
        const ret = wasm.coordinate_new(x, y);
        return Coordinate.__wrap(ret);
    }
}
/**
*/
export class Piece {

    static __wrap(ptr) {
        const obj = Object.create(Piece.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_piece_free(ptr);
    }
    /**
    * @returns {number}
    */
    get team() {
        const ret = wasm.__wbg_get_piece_team(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set team(arg0) {
        wasm.__wbg_set_piece_team(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get piece_type() {
        const ret = wasm.__wbg_get_piece_piece_type(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set piece_type(arg0) {
        wasm.__wbg_set_piece_piece_type(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get health() {
        const ret = wasm.__wbg_get_coordinate_x(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set health(arg0) {
        wasm.__wbg_set_coordinate_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    relative_health() {
        const ret = wasm.piece_relative_health(this.ptr);
        return ret;
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_x_f80904619bd3e1b2 = function(arg0) {
        const ret = getObject(arg0).x;
        return ret;
    };
    imports.wbg.__wbg_y_1c6ec6da0ad6377d = function(arg0) {
        const ret = getObject(arg0).y;
        return ret;
    };
    imports.wbg.__wbg_log_72dcb5188e974f1f = function(arg0) {
        console.log(arg0 === 0 ? undefined : Piece.__wrap(arg0));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('chess_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
