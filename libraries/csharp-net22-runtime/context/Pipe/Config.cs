
using System;
using Newtonsoft.Json;
using System.IO;

namespace Data
{
    public class Config
    {
        public string Algoname { get; }
        public string Username { get; }
        
        public string Algopath { get; }

        public Config(string sysPath)
        {
            string inputPath = Path.Combine(sysPath, "algorithmia.conf");
            Console.WriteLine(inputPath);
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