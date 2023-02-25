import { Application } from "https://deno.land/x/oak@v9.0.0/application.ts";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import { router } from "./routes/router.ts";
import * as colors from "https://deno.land/std@0.177.0/fmt/colors.ts";
import { DISTRIB_CONFIG, TS_REST_CONFIG } from "./utils/config.ts";

const app = new Application();
app.use(oakCors({ origin: true }));
app.use(router.allowedMethods());
app.use(router.routes());

app.addEventListener("listen", ({ hostname, port, serverType }) => {
    console.log(
        colors.bold("Start listening on ") + colors.yellow(`${hostname}:${port}`),
    );

    console.log(
        colors.bold("Expecting distrib at ") + colors.yellow(`${DISTRIB_CONFIG.host}:${DISTRIB_CONFIG.port}`),
    );
    
    console.log(colors.bold("  using HTTP server: " + colors.yellow(serverType)));
});

await app.listen({ hostname: TS_REST_CONFIG.host, port: TS_REST_CONFIG.port });