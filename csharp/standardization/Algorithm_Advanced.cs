using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using Algo.Devtools;
using Algorithmia;
using AlgorithmiaPipe;
//This is an example of an algorithm. You can test it by copy/pasting the following into the web editor:
/// {"name": "Algorithmia user"}
///
/// You should expect to see the following as output:
/// {"output":"Hello Algorithmia user"}

/// 'Algo' is the primary namespace of your algorithm.
namespace Algo
{
    /// The input object type for your algorithm, please ensure that your class members are public and simple types (string, list, int, etc)
    ///
    /// Note: json serialization is 'Case Sensitive', this means that for certain types of input - it might make sense to break
    /// from C# convention, and name your AlgoInput/AlgoOutput variables in 'snake_case' format. (https://en.wikipedia.org/wiki/Snake_case)
    public class AlgoInput : Object
    {
        public string image_path;
    }

    /// This is the output object type for your algorithm, the types defined here must be json serializable via the 'Newtonsoft.Json' package.
    public class AlgoOutput : Object
    {
        public string save_path;
    }

    /// This is the main class of your algorithm, it contains your static 'apply' method which we use as an entry point to your project.
    public class AlgorithmAdvanced
    {
        public static AlgoOutput Apply(AlgoInput input, Dictionary<string, object> context)
        {
            string randomeFileName = RandomString(10);
            string filename = $"{randomeFileName}.jpg";
            string localFilePath = $"/tmp/{filename}";
            string remoteFilePath = $"data://.my/collection/{filename}";
            FileStream stream = File.OpenRead(localFilePath);
            Client client = (Client) context["client"];
            client.file($"data://.my/collection/{filename}").put(stream);
            stream.Close();
            AlgoOutput output = new AlgoOutput();
            output.save_path = remoteFilePath;
            return output;
        }
        
        public static string RandomString(int length)
        {
            Random random = new Random();
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            return new string(Enumerable.Repeat(chars, length)
                .Select(s => s[random.Next(s.Length)]).ToArray());
        }


        public static Dictionary<String, dynamic> OnLoad()
        {
            Dictionary<String, dynamic> context = new Dictionary<string, dynamic>();
            context["client"] = new Client(Environment.GetEnvironmentVariable("ALGORITHMIA_API_KEY"));
            return context;
        }

        public static AlgorithmHandler<AlgoInput, AlgoOutput> SetupHandler()
        {
            AlgorithmHandler<AlgoInput, AlgoOutput> handler = new AlgorithmHandler<AlgoInput, AlgoOutput>();
            handler.SetLoadFunction(OnLoad);
            handler.SetApplyFunction(Apply);
            return handler;
        }

    }
}
