using System;
using System.Collections.Generic;
using System.Collections.Specialized;
using System.IO;
using System.IO.Pipes;
using System.Security.Cryptography.X509Certificates;
using Newtonsoft.Json;

namespace Pipe
{    public class ProcessIO
    {
        public string ContentType {get;}
        public string Data { get; }
        private static string OutputPath = "/tmp/algoout";

        public static void WriteToPipe(object data)
        {
            string serialized = JsonConvert.SerializeObject(data);
            using (StreamWriter w = new StreamWriter(OutputPath))
            {
                w.Write(serialized);
            }
        }

        public ProcessIO()
        {
            string input;
            List<string> stdinBuilder = new List<string>();
            while ((input = Console.In.ReadLine()) != null && input != "")
            {
                stdinBuilder.Add(input);
            }

            string fullInput = String.Join(String.Empty, stdinBuilder);
            dynamic jobj = JsonConvert.DeserializeObject(fullInput);
            if (jobj["content_type"] == "json" || jobj["content_type"] == "string" || jobj["content_type"] == "binary")
            {
                Data = jobj["data"].ToString();
                ContentType = jobj["content_type"].ToString();
            }
            else
            {
                throw new Exception("request's content type is invalid.");
            }
        }
    }
}