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
        static public AlgoOutput apply(AlgoInput input)
        {
            string name = input.name;
            AlgoOutput output = new AlgoOutput {output = "Hello " + name}; 
            return output;
        }
    }
}
