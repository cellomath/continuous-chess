/* tslint:disable */
/* eslint-disable */
/**
*/
export enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}
/**
*/
export enum PieceTeam {
  White,
  Black,
}
/**
*/
export class Board {
  free(): void;
/**
*/
  constructor();
/**
* @param {any} from
* @param {any} to
* @returns {boolean}
*/
  movePiece(from: any, to: any): boolean;
/**
* @param {any} position
* @returns {Piece | undefined}
*/
  get_piece_at(position: any): Piece | undefined;
/**
*/
  run_damage_calculation(): void;
}
/**
*/
export class Coordinate {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  constructor(x: number, y: number);
/**
*/
  x: number;
/**
*/
  y: number;
}
/**
*/
export class Piece {
  free(): void;
/**
* @returns {number}
*/
  relative_health(): number;
/**
*/
  health: number;
/**
*/
  piece_type: number;
/**
*/
  team: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_coordinate_free: (a: number) => void;
  readonly __wbg_get_coordinate_x: (a: number) => number;
  readonly __wbg_set_coordinate_x: (a: number, b: number) => void;
  readonly __wbg_get_coordinate_y: (a: number) => number;
  readonly __wbg_set_coordinate_y: (a: number, b: number) => void;
  readonly coordinate_new: (a: number, b: number) => number;
  readonly __wbg_get_piece_team: (a: number) => number;
  readonly __wbg_set_piece_team: (a: number, b: number) => void;
  readonly __wbg_get_piece_piece_type: (a: number) => number;
  readonly __wbg_set_piece_piece_type: (a: number, b: number) => void;
  readonly piece_relative_health: (a: number) => number;
  readonly __wbg_board_free: (a: number) => void;
  readonly board_default_setup: () => number;
  readonly board_movePiece: (a: number, b: number, c: number) => number;
  readonly board_get_piece_at: (a: number, b: number) => number;
  readonly board_run_damage_calculation: (a: number) => void;
  readonly __wbg_set_piece_health: (a: number, b: number) => void;
  readonly __wbg_get_piece_health: (a: number) => number;
  readonly __wbg_piece_free: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
