using Algorithmia;
using __ALGO__;
// This file takes your __ALGO__.cs file, loads the apply function and creates a Pipe wrapper around it.
// WARNING: Do not remove this file from your project, it will make your algorithm unusable.

namespace __ALGO__

{
    public class pipe
    {
        static int Main()
        {
            return AlgorithmiaPipe.Pipe.Enter(typeof(__ALGO__));
        }
    }
}