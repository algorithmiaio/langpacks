using System;
using System.Collections.Generic;
using Algo.Devtools;
using AlgorithmiaPipe;

namespace Algo
{
        public class AlgorithmBasic
        {
            public static string Foo(string input)
            {
                return $"Hello {input}";
            }

            public static void Main(string[] args)
            {
                var algo = new AlgorithmHandler<String, Dictionary<String, Object>, String>(Foo);
                algo.Run();
            }
    }
}