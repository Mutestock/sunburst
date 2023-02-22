import { router as basic_router } from "./routes/basic_routes.js";
import { router as article_router } from "./routes/article_routes.js";

import { JS_REST_CONFIG } from "./utils/config.js";
import express from "express";

let app = express();
const port = JS_REST_CONFIG.port;
const host = JS_REST_CONFIG.host;

app.use(basic_router);
app.use(article_router);

app.listen(port, host, function (err) {
    if (err) {
        console.log(err);
    }
    console.log(`Javascript Express listening on ${host}:${port}`);
})

