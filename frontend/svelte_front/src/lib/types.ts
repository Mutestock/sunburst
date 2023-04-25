export class ArticleStatsProfile {
    name: string;
    searchTerm: string;
    articleCount: ArticleCount

    constructor(name: string, searchTerm: string, articleCount: ArticleCount) {
        this.name = name;
        this.searchTerm = searchTerm;
        this.articleCount = articleCount;
    }
}

export enum Casing {
    CamelCase,
    SnakeCase,
};

export class ArticleCount{
    total: number;
    cntMatches: number;
    cntNonMatches: number;

    constructor(total: number, cntMatches: number, cntNonMatches: number){
        this.total = total;
        this.cntMatches = cntMatches;
        this.cntNonMatches = cntNonMatches;
    }

    static async fromResponse(response: Response, casing: Casing): Promise<ArticleCount> {
        let total = 0;
        let cntMatches = 0;
        let cntNonMatches = 0;

        let resJson = await response.json();

        if (casing == Casing.CamelCase){
            total = resJson.total,
            cntMatches = resJson.cntContainedSearchTerm;
            cntNonMatches = resJson.cntNotContainedSearchTerm;
        } 
        else if (casing == Casing.SnakeCase){
            total = resJson.total,
            cntMatches = resJson.cnt_contained_search_term;
            cntNonMatches = resJson.cnt_not_contained_search_term;
        }

        return new ArticleCount(total, cntMatches, cntNonMatches);
    }
}

export class RestServiceOptions{
    name: string;
    url: string;
    country_code: string;
    casing: Casing

    constructor(name: string, url: string, country_code: string, casing: Casing){
        this.name = name;
        this.url = url;
        this.country_code = country_code;
        this.casing = casing;
    }
}