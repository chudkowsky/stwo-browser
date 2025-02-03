import * as wasm from "./login_bg.wasm";
export * from "./login_bg.js";
import { __wbg_set_wasm } from "./login_bg.js";
__wbg_set_wasm(wasm);