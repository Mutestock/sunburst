import toml from "toml";
import fs from "fs";

const CONFIG_PATH = "../../config.toml";
const DATA = fs.readFileSync(CONFIG_PATH, "utf8");
const CONFIG = toml.parse(DATA)

let distrib_config = {}
let js_rest_config = {}

if (process.env.PRODUCTION){
    distrib_config = CONFIG.distributor.prod
    js_rest_config = CONFIG.rest.js.prod
}
else if (process.env.CONTAINERIZED){
    distrib_config = CONFIG.distributor.containerized
    js_rest_config = CONFIG.rest.js.prod
}
else {
    distrib_config = CONFIG.distributor.dev
    js_rest_config = CONFIG.rest.js.dev
}

const JS_REST_CONFIG = js_rest_config
const DISTRIB_CONFIG = distrib_config


export {
    CONFIG, JS_REST_CONFIG, DISTRIB_CONFIG
}