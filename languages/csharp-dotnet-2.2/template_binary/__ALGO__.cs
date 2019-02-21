using System;
using System.IO;

using OpenCvSharp;

namespace __ALGO__
{
    
    public class AlgoOutput
    {
        public string output;
    }
    
    class __ALGO__
    {
        static public AlgoOutput apply(byte[] input)
        {
            string hex = BitConverter.ToString(input).Replace("-", "");
            AlgoOutput output = new AlgoOutput {output = $"input is {hex} as hex"}; 
            return output;
        }
        static void Main()
        {
            byte[] stream = File.ReadAllBytes("/home/zeryx/Downloads/profile.jpeg");
            string test = Convert.ToBase64String(stream);
            byte[] deserialized = Convert.FromBase64String(test);
            
            AlgoOutput output = apply(deserialized);
        }
    }
}