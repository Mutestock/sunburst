import * as protoLoader from "npm:@grpc/proto-loader";
import * as grpc from "npm:@grpc/grpc-js";
import { DISTRIB_CONFIG } from "../utils/config.ts";
import { ArticleCount } from "../models/article_count.ts";

const PROTO_PATH = "../../../proto/article.proto";
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

const client = new articleService.ArticleService(`${DISTRIB_CONFIG.host}:${DISTRIB_CONFIG.port}`, grpc.credentials.createInsecure());

async function readArticles(): Promise<[grpc.GrpcObject]> {
    return await new Promise((resolve, reject) => {
        client.readArticleList({}, (err: Error, articles: [grpc.GrpcObject]) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}

async function readArticlesBySite(site: string): Promise<[grpc.GrpcObject]> {
    return await new Promise((resolve, reject) => {
        client.readArticleListBySite({ site: site }, (err: Error, articles: [grpc.GrpcObject]) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
                
            }
        });
    });
}

async function readArticlesBySearchterm(searchTerm: string): Promise<[grpc.GrpcObject]> {
    return await new Promise((resolve, reject) => {
        client.readArticleListBySearchterm({ search_term: searchTerm }, (err: Error, articles: [grpc.GrpcObject]) => {
            if (err) {
                reject(err);
            } else {
                resolve(articles);
            }
        });
    });
}

async function readArticleCountBySearchSite(site: string, searchTerm: string): Promise<ArticleCount> {
    return await new Promise((resolve, reject) => {
        client.readArticleCountBySearchSite({ site: site, search_term: searchTerm }, (err: Error, articles: ArticleCount) => {
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