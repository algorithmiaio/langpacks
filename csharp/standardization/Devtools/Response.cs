using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using Newtonsoft.Json;
namespace AlgorithmiaPipe
{
    public class Response
    {
        public class MetaData
        {
            public string content_type;

            public MetaData(string contentType)
            {
                content_type = contentType;
            }
        }

        public readonly object result;
        public readonly MetaData metadata;

        public Response(dynamic result)
        {
            string content;
            object data;
            switch (result)
            {
                case string _:
                    content = "text";
                    data = result;
                    break;
                case byte[] bytes:
                    content = "binary";
                    data = Convert.ToBase64String(bytes);
                    break;
                default:
                    content = "json";
                    data = result;
                    break;
            }

            metadata = new MetaData(content);
            this.result = data;
        }
    }
}