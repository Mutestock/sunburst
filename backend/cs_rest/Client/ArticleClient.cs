using Article;
using cs_rest.Utils;
using Grpc.Net.Client;

namespace cs_rest.Client
{
    public class ArticleClient
    {
        public static async Task<ReadArticleListResponse> ReadArticles()
        {
            var channel = GrpcChannel.ForAddress(Config.MakeGrpcClientAddress());
            var client = new ArticleService.ArticleServiceClient(channel);
            var response = await client.ReadArticleListAsync(new ReadArticleListRequest { });
            return response;
        }

        public static async Task<ReadArticleListBySiteResponse> ReadArticlesBySite(string site)
        {
            var channel = GrpcChannel.ForAddress(Config.MakeGrpcClientAddress());
            var client = new ArticleService.ArticleServiceClient(channel);
            var response = await client.ReadArticleListBySiteAsync(
                new ReadArticleListBySiteRequest { Site = site }
            );
            return response;
        }

        public static async Task<ReadArticleListBySearchtermResponse> ReadArticlesBySearchTerm(string searchTerm)
        {
            var channel = GrpcChannel.ForAddress(Config.MakeGrpcClientAddress());
            var client = new ArticleService.ArticleServiceClient(channel);
            var response = await client.ReadArticleListBySearchtermAsync(
                new ReadArticleListBySearchtermRequest { SearchTerm = searchTerm }
            );
            return response;
        }

        public static async Task<ReadArticleCountBySearchSiteResponse> ReadArticleCountBySearchSite(string site, string searchTerm)
        {
            var channel = GrpcChannel.ForAddress(Config.MakeGrpcClientAddress());
            var client = new ArticleService.ArticleServiceClient(channel);
            var response = await client.ReadArticleCountBySearchSiteAsync(
                new ReadArticleCountBySearchSiteRequest { Site = site, SearchTerm = searchTerm }
            );
            return response;
        }
    }
}