using Article;
using cs_rest.Client;
using Google.Protobuf.Collections;
using Microsoft.AspNetCore.Mvc;

namespace cs_rest.Controllers;

[ApiController]
public class ArticlesController : ControllerBase
{

    private readonly ILogger<ArticlesController> _logger;

    public ArticlesController(ILogger<ArticlesController> logger)
    {
        _logger = logger;
    }

    [HttpGet]
    [Route("/article")]
    public async Task<RepeatedField<ArticleMessage>> ReadArticlesRoute()
    {
        return (await ArticleClient.ReadArticles()).Articles;
    }

    [HttpGet]
    [Route("/article/site={site}")]
    public async Task<RepeatedField<ArticleMessage>> ReadArticlesBySiteRouteAsync(string site)
    {
        return (await ArticleClient.ReadArticlesBySite(site)).Articles;
    }

    [HttpGet]
    [Route("/article/search-term={searchTerm}")]
    public async Task<RepeatedField<ArticleMessage>> ReadArticlesBySearchtermRoute(string searchTerm)
    {
        return (await ArticleClient.ReadArticlesBySearchTerm(searchTerm)).Articles;
    }


    [HttpGet]
    [Route("/article/count/site={site}/search={searchTerm}")]
    public async Task<ReadArticleCountBySearchSiteResponse> ReadArticleCountBySearchSiteRoute(string site, string searchTerm)
    {
        return (await ArticleClient.ReadArticleCountBySearchSite(site, searchTerm));
    }
}
