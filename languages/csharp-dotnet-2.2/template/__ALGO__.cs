using System;
using Algorithmia;
using Newtonsoft.Json;

namespace __ALGO__
{
    
    public class AlgoInput
    {
        public string Name;        
    }

    public class AlgoOutput
    {
        public string Output;
    }
    
    class __ALGO__
    {
        static public AlgoOutput Apply(AlgoInput input)
        {
            string name = input.Name;
            AlgoOutput output = new AlgoOutput {Output = "Hello " + name}; 
            return output;
        }

        static void Main()
        {
            AlgoInput test = new AlgoInput {Name = "James"};
            AlgoOutput output = Apply(test);
            Console.WriteLine(JsonConvert.SerializeObject(output));
        }
    }
    
}