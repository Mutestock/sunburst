import { Router } from "https://deno.land/x/oak@v9.0.0/router.ts";
import { articleRoutes } from "./article_routes.ts";
import { basicRoutes } from "./basic_routes.ts";

const router = new Router();

basicRoutes(router);
articleRoutes(router);

export {
    router
}