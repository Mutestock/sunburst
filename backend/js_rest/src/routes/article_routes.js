import { Router } from "express";
import { readArticles, readArticlesBySite, readArticlesBySearchterm, readArticleCountBySearchSite} from "../client/article_grpc_client.js";

let router = Router();

router.get('/article', async function (_req, res, _next) {
    res.send(await readArticles());
});

router.get("/article/site=:site", async function (req, res, _next) {
    res.send(await readArticlesBySite(req.params.site))
});

router.get("/article/search-term=:searchTerm", async function (req, res, _next) {
    res.send(await readArticlesBySearchterm(req.params.searchTerm))
});

router.get("/article/count/site=:site/search=:searchTerm", async function (req, res, _next) {
    console.log(req.params.site);
    console.log(req.params.searchTerm);
    res.send(await readArticleCountBySearchSite(req.params.site, req.params.searchTerm))
});


export {
    router
}