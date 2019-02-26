using System;
using Algorithmia;

//This is an example of an algorithm, you can evaluate it with this input:
/// Example Input:
/// {"name": "Algorithmia user"}
///
/// Expected Output:
/// {"output":"Hello Algorithmia user"}
namespace __ALGO__
{
    public class AlgoInput
    {/// <summary>
     /// The input object type for your algorithm, this must contain json serializable types.
     /// </summary>
        public string name;
    }

    public class AlgoOutput
    {
        /// <summary>
        /// This is the output object type for your algorithm, it can contain optional types, but they all must be json serializable.
        /// </summary>
        public string output;
    }
    
    class __ALGO__
    {
        /// <summary>
        /// API calls will begin at the apply() method, with the request body passed as 'input'
        /// For more details, see algorithmia.com/developers/algorithm-development/languages
        /// </summary>
        /// <param name="input"></param>
        /// <returns></returns>
        static public AlgoOutput apply(AlgoInput input)
        {
            string name = input.name;
            AlgoOutput output = new AlgoOutput {output = "Hello " + name}; 
            return output;
        }
    }
}
