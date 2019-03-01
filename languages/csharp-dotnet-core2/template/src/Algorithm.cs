using System;
using Algorithmia;

//This is an example of an algorithm. You can test it by copy/pasting the following into the web editor:
/// {"name": "Algorithmia user"}
///
/// You should expect to see the following as output:
/// {"output":"Hello Algorithmia user"}



/// 'Algo' is the primary namespace of your algorithm.
namespace Algo
{
    /// The input object type for your algorithm, the types defined here must be json serializable via the 'Newtonsoft.Json' package.
    /// Note: json serialization is 'Case Sensitive', this means that for certain types of input - it might make sense to break
    /// from C# convention, and name your AlgoInput/AlgoOutput variables in 'snake_case' format. (https://en.wikipedia.org/wiki/Snake_case)
    public class AlgoInput
    {
        
        public string name;
    }

    /// This is the output object type for your algorithm, the types defined here must be json serializable via the 'Newtonsoft.Json' package.
    public class AlgoOutput
    {
        public string greeting;
    }
    
    /// This is the main class of your algorithm, it contains your static 'apply' method which we use as an entry point to your project.
    class Algorithm
    {
        /// API calls will begin at the apply() method, with the request body passed as 'input'
        /// For more details, see algorithmia.com/developers/algorithm-development/languages
        public static AlgoOutput apply(AlgoInput input)
        {
            string name = input.name;
            AlgoOutput output = new AlgoOutput {greeting = "Hello " + name}; 
            return output;
        }
    }
}
