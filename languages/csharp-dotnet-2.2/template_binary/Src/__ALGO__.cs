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
            throw new Exception("failure testing.");
            return output;
        }
    }
}

