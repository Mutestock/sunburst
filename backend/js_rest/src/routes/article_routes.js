import { Router } from "express";

let router = Router();

router.get('/article', async function (req, res, next) {
    res.send('Ok');
});

router.get("/article/site=:site", async function (req, res, next) {
    res.send(`Site was ${req.params.site}`)
});

router.get("/article/search-term=:searchTerm", async function (req, res, next) {
    res.send(`Search term was ${req.params.searchTerm}`)
});

router.get("/article/count/site=:site/search=:searchTerm", async function (req, res, next) {
    res.send(`Site was ${req.params.site} and search term was ${req.params.searchTerm}`)
});


export {
    router
}
