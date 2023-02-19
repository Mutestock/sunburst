using Tomlyn;
using Tomlyn.Model;

namespace cs_rest.Utils
{
    public class Config
    {
        public static TomlTable GetConfig()
        {
            string text = System.IO.File.ReadAllText("../../config.toml");
            return Toml.ToModel(text);
        }

        public static string MakeGrpcClientAddress()
        {
            string address = "";
            if (Environment.GetEnvironmentVariable("PRODUCTION") == "1")
            {
                address = String.Format("http://{0}:{1}",
                    ((TomlTable)((TomlTable)GetConfig()["distributor"]!)["prod"]!)["host"],
                    ((TomlTable)((TomlTable)GetConfig()["distributor"]!)["prod"]!)["port"]
                );
            }
            else if (Environment.GetEnvironmentVariable("CONTAINERIZED") == "1"){
                address = String.Format("http://{0}:{1}",
                    ((TomlTable)((TomlTable)GetConfig()["distributor"]!)["containerized"]!)["host"],
                    ((TomlTable)((TomlTable)GetConfig()["distributor"]!)["containerized"]!)["port"]
                );
            }
            else{
                address = String.Format("http://{0}:{1}",
                    "localhost",
                    ((TomlTable)((TomlTable)GetConfig()["distributor"]!)["dev"]!)["port"]
                );
            }
            return address;
        }
    }
}