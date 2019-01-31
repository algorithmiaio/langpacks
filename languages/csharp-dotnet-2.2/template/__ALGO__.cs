using System;
using Algorithmia;
using Newtonsoft.Json;

namespace __ALGO__
{
    
    public class AlgoInput
    {
        public string name;        
    }

    public class AlgoOutput
    {
        public string output;
    }
    
    class __ALGO__
    {
        static AlgoOutput apply(AlgoInput input)
        {
            string name = input.name;
            AlgoOutput output = new AlgoOutput {output = "Hello " + name}; 
            return output;
        }
        static void Main()
        {
            AlgoInput test = new AlgoInput {name = "James"};
            AlgoOutput output = apply(test);
            Console.WriteLine(JsonConvert.SerializeObject(output));
        }
    }
    
}