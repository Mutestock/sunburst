from dataclasses import dataclass


@dataclass
class ArticleCount:
    total: int
    cnt_contained_search_term: int
    cnt_not_contained_search_term: int

    @classmethod
    def from_response(cls, response):
        return cls(
            total=response.total,
            cnt_contained_search_term=response.cnt_contained_search_term,
            cnt_not_contained_search_term=response.cnt_not_contained_search_term,
        )
