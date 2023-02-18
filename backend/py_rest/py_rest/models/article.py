from dataclasses import dataclass
from proto_implementations.protogen.article_pb2 import ArticleMessage
from dataclasses import dataclass


@dataclass
class Article:
    name: str
    site: str
    url: str
    language: str
    scrape_date: str
    submission_date: str
    tags_and_categories: list

    @classmethod
    def from_article_message(cls, article_message: ArticleMessage):
        name = ""
        site = ""
        url = ""
        language = ""
        scrape_date = ""
        submission_date = ""
        tags_and_categories = []

        if article_message.name:
            name = str(article_message.name)
        if article_message.site:
            site = str(article_message.site)
        if article_message.url:
            url = str(article_message.url)
        if article_message.language:
            language = str(article_message.language)
        if article_message.scrape_date:
            scrape_date = str(article_message.scrape_date)
        if article_message.submission_date:
            submission_date = str(article_message.submission_date)
        if article_message.tags_and_categories:
            if type(article_message.tags_and_categories) == str:
                tags_and_categories = [str(article_message.tags_and_categories)]
            else:
                tags_and_categories = [
                    str(item) for item in article_message.tags_and_categories
                ]

        return cls(
            name,
            site,
            url,
            language,
            scrape_date,
            submission_date,
            tags_and_categories,
        )



