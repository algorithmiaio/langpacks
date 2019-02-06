using System;
using System.Collections.Generic;
using System.Collections.Specialized;
using System.IO;
using System.IO.Pipes;
using System.Security.Cryptography.X509Certificates;
using Newtonsoft.Json;
namespace Pipe
{    public class Write
    {
        private static string OutputPath = "/tmp/algoout";

        public static void WriteJsonToPipe(object response)
        {
            Console.Out.Flush();
            string serialized = JsonConvert.SerializeObject(response);
            using (StreamWriter w = new StreamWriter(OutputPath))
            {
                w.Write(serialized);
            }
        }
    }
}