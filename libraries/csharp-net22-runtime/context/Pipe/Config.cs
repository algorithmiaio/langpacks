
using Newtonsoft.Json;
using System.IO;

namespace Pipe
{
    public class Config
    {
        public string AlgoName { get; set; }

        public Config()
        {
        string CONFIG_PATH = "algorithmia.conf";
        using (StreamReader r = new StreamReader(CONFIG_PATH))
        {
            string json = r.ReadToEnd();
            dynamic array = JsonConvert.DeserializeObject(json);
            return array;
        }
        }
    }
}