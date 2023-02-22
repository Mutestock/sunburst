import * as protoLoader from "@grpc/proto-loader";
import * as grpc from "@grpc/grpc-js";

const PROTO_PATH = "../../proto/article.proto";
const PACKAGE_DEFINITION = protoLoader.loadSync(
    PROTO_PATH,
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true
    }
);
const PROTO_DESCRIPTOR = grpc.loadPackageDefinition(PACKAGE_DEFINITION);
const articleService = PROTO_DESCRIPTOR.article;
let client = new articleService.ArticleService("localhost:20488", grpc.credentials.createInsecure());

async function readArticles() {
    return new Promise((resolve, reject) => {
        client.readArticleList({}, (err, articles) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}

async function readArticlesBySite(site) {
    return new Promise((resolve, reject) => {
        client.readArticleListBySite({ site: site }, (err, articles) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}

async function readArticlesBySearchterm(searchTerm) {
    return new Promise((resolve, reject) => {
        client.readArticleListBySearchterm({ search_term: searchTerm }, (err, articles) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}

async function readArticleCountBySearchSite(site, searchTerm) {
    return new Promise((resolve, reject) => {
        client.readArticleCountBySearchSite({ site: site, search_term: searchTerm }, (err, articles) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}


export {
    readArticles, readArticlesBySite, readArticlesBySearchterm, readArticleCountBySearchSite
}