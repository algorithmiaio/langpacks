using System.ComponentModel.DataAnnotations;
using Newtonsoft.Json;
namespace Pipe
{
    public class Response
    {
        public class MetaData
        {
            public string content_type { get; }

            public MetaData(string contentType)
            {
                content_type = contentType;
            }
        }
        
        public object result;
        public MetaData metadata;

        public Response(object res, string content)
        {
            metadata = new MetaData(content);
            result = res;
        }
    }
}