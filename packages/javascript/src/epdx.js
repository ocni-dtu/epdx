import * as wasm from "./epdx_bg.wasm";
import { __wbg_set_wasm } from "./epdx_bg.js";
__wbg_set_wasm(wasm);
export * from "./epdx_bg.js";
