using System;
using Newtonsoft.Json;
using System.IO;
namespace Pipe
{
    public class Request
    {
        public string ContentType;
        public string Data;
        
        public Request(string input)
        {
            dynamic jobj = JsonConvert.DeserializeObject(input);
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