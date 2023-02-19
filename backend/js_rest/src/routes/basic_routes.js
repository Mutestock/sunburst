import { Router } from "express";

let router = Router();

router.get('/health', async function (req, res, next) {
    res.send('Ok');
});


export {
    router
}
