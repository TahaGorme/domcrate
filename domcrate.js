/* @ts-self-types="./domcrate.d.ts" */
import * as wasm from "./domcrate_bg.wasm";
import { __wbg_set_wasm } from "./domcrate_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    create_stuff, run_alert
} from "./domcrate_bg.js";
