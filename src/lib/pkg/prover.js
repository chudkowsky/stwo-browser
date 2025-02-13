import * as wasm from "./prover_bg.wasm";
export * from "./prover_bg.js";
import { __wbg_set_wasm } from "./prover_bg.js";
__wbg_set_wasm(wasm);