using System;
using System.Collections.Generic;
using Algo.Devtools;
using AlgorithmiaPipe;

namespace Algo
{
        public class AlgorithmBasic
        {
            public static string Foo(string input, Dictionary<String, object> context = null)
            {
                return $"Hello {input}";
            }

            public static AlgorithmHandler<string, string> SetupHandler()
            {
                AlgorithmHandler<string, string> handler = new AlgorithmHandler<string, string>();
                handler.SetApplyFunction(Foo);
                return handler;
            }
    }
}