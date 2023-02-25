import { Router } from "https://deno.land/x/oak@v9.0.0/mod.ts";


function basicRoutes(router: Router): Router {
    router.get("/health", (context) => {
        context.response.body = "Ok"
    });

    return router;
}

export {
    basicRoutes
}
