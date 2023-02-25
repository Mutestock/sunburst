import { Router } from "https://deno.land/x/oak@v9.0.0/mod.ts";
import { readArticleCountBySearchSite, readArticles, readArticlesBySearchterm, readArticlesBySite } from "../client/article_client.ts";

const BASE_ROUTE = "/article"

function articleRoutes(router: Router): Router {
    router.get(BASE_ROUTE, async (context) => {
        const content = await readArticles();
        console.log(content);
        
        context.response.body = content
    });

    router.get<{ site: string }>(BASE_ROUTE + "/:site", async (context) => {
        const content = await readArticlesBySite(context.params.site);
        context.response.body = content
    });

    router.get<{ searchTerm: string }>(BASE_ROUTE + "/:searchTerm", async (context) => {
        const content = await readArticlesBySearchterm(context.params.searchTerm);
        context.response.body = content
    });

    router.get<{ site: string, searchTerm: string }>(BASE_ROUTE + "/count/site=:site/search=:searchTerm",
        async (context) => {
            const content = await readArticleCountBySearchSite(context.params.site, context.params.searchTerm);
            context.response.body = content
        }
    );

    return router;
}

export {
    articleRoutes
}
