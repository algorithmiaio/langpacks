using System;
using System.IO;
using System.IO.Pipes;
using Newtonsoft.Json;
namespace AlgorithmiaPipe
{    public class Write
    {
        private static string OutputPath = "/tmp/algoout";

        public static void WriteJsonToPipe(object response)
        {
            Console.Out.Flush();
            var fs = File.OpenWrite(OutputPath);
            string serialized = JsonConvert.SerializeObject(response);
            using (StreamWriter w = new StreamWriter(fs))
            {
                w.Write(serialized);
                w.Write("\n");
                w.Flush();
            }
        }
    }
}
