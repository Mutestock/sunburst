import toml from "npm:toml@3.0.0";

const CONFIG_PATH = "../../../config.toml";
const DATA = await Deno.readTextFile(CONFIG_PATH);
const CONFIG = toml.parse(DATA)

let distrib_config = {}
let ts_rest_config = {}

if (Deno.env.get("PRODUCTION")){
    distrib_config = CONFIG.distributor.prod
    ts_rest_config = CONFIG.rest.deno.prod
}
else if (Deno.env.get("CONTAINERIZED")){
    distrib_config = CONFIG.distributor.containerized
    ts_rest_config = CONFIG.rest.deno.prod
}
else {
    distrib_config = CONFIG.distributor.dev
    ts_rest_config = CONFIG.rest.deno.dev
}

// deno-lint-ignore no-explicit-any
const TS_REST_CONFIG: any = ts_rest_config
// deno-lint-ignore no-explicit-any
const DISTRIB_CONFIG: any = distrib_config


export {
    CONFIG, TS_REST_CONFIG, DISTRIB_CONFIG
}
