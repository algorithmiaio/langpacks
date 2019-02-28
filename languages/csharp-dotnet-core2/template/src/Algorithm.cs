using System;
using Algorithmia;

//This is an example of an algorithm, you can evaluate it with this input:
/// Example Input:
/// {"name": "Algorithmia user"}
///
/// Expected Output:
/// {"output":"Hello Algorithmia user"}
namespace Algorithm
{
    public class AlgoInput
    {
        
     /// The input object type for your algorithm, this must contain json serializable types.
        public string name;
    }

    public class AlgoOutput
    {
        /// This is the output object type for your algorithm, it can contain optional types, but they all must be json serializable.
        public string output;
    }
    
    class Algorithm
    {
        /// API calls will begin at the apply() method, with the request body passed as 'input'
        /// For more details, see algorithmia.com/developers/algorithm-development/languages
        public static AlgoOutput apply(AlgoInput input)
        {
            string name = input.name;
            AlgoOutput output = new AlgoOutput {output = "Hello " + name}; 
            return output;
        }
    }
}
