use crate::tonic_proto_out::ReadArticleCountBySearchSiteResponse;
use chrono::NaiveDateTime;
use sunburst_models::{article::Article, article_count::ArticleCount};

use crate::tonic_proto_out::ArticleMessage;

pub trait ArticleGrpcBind {
    fn to_article(self) -> Article;
}

impl ArticleGrpcBind for ArticleMessage {
    fn to_article(self) -> Article {
        Article {
            name: self.name,
            site: self.site,
            url: self.url,
            country_code: self.country_code,
            scrape_date: NaiveDateTime::parse_from_str(&self.scrape_date, "%Y-%m-%d %H:%M:%S%.f")
                .expect("Could not parse scrape date to naivedatetime"),
            submission_date: match self.submission_date {
                Some(v) => Some(
                    NaiveDateTime::parse_from_str(&v, "%Y-%m-%d %H:%M:%S%.f")
                        .expect("Could not parse submission date to naivedatetime"),
                ),
                None => None,
            },

            tags_and_categories: self.tags_and_categories,
        }
    }
}

pub trait ArticlesGrpcBind {
    fn to_articles(self) -> Vec<Article>;
}

impl ArticlesGrpcBind for Vec<ArticleMessage> {
    fn to_articles(self) -> Vec<Article> {
        self.into_iter().map(|x| x.to_article()).collect()
    }
}

pub trait ArticleCountBind {
    fn to_stats(self) -> ArticleCount;
}

impl ArticleCountBind for ReadArticleCountBySearchSiteResponse {
    fn to_stats(self) -> ArticleCount {
        ArticleCount {
            total: self.total,
            cnt_contained_search_term: self.cnt_contained_search_term,
            cnt_not_contained_search_term: self.cnt_not_contained_search_term,
        }
    }
}
