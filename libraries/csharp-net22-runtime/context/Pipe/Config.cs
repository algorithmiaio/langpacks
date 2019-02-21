
using System;
using Newtonsoft.Json;
using System.IO;

namespace Pipe
{
    public class Config
    {
        public string Algoname;
        public string Username;
        public string Algopath;

        public Config(string sysPath)
        {
            string inputPath = Path.Combine(sysPath, "algorithmia.conf");
            using (StreamReader r = new StreamReader(inputPath))
            {
                string json = r.ReadToEnd();
                dynamic array = JsonConvert.DeserializeObject(json);
                Algoname = array["algoname"];
                Username = array["username"];
                Algopath = sysPath;
            }
        }
    }
}