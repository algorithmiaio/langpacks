using Algorithmia;
using Algorithm;
// This file takes your Algorithm.cs file, loads the apply function and creates a Pipe wrapper around it.
// WARNING: Do not remove this file from your project, it will make your algorithm unusable.

namespace Algorithm

{
    public class Pipe
    {
        static int Main()
        {
            return AlgorithmiaPipe.Pipe.Enter(typeof(Algorithm));
        }
    }
}