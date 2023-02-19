
using Microsoft.AspNetCore.Mvc;

namespace cs_rest.Controllers
{
    [ApiController]
    public class BasicController: ControllerBase
    {
        [HttpGet]
        [Route("/health")]
        public string HealthCheck(){
            return "ok";
        }
    }
}