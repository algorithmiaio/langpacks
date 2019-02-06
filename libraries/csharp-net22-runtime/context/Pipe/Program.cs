using System;
using System.Reflection;
using Pipe;

namespace Pipe
{
    class Program
    {


        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                throw new Exception(
                    "no algorithm directory argument found. Please provide the path to a valid C# algorithm.");
            }
            string sysPath = args[0];
            Config config = new Config(sysPath);
            Module algoModule = new Module(config);
            Console.Out.WriteLine("PIPE_INIT_COMPLETE");
            
            string readLine;
            while ((readLine = Console.In.ReadLine()) != null && readLine != "")
            {
                Request request = new Request(readLine);
                object response = null;
                try
                {
                    object result = algoModule.AttemptExecute(request);
                     response = new Response(result, "json");
                }
                catch (Exception e)
                {
                    response = new ExceptionResponse(e);
                }
                finally
                {
                    Write.WriteJsonToPipe(response);
                }
            }
            Console.WriteLine("PIPE_TERMINATE");
        }
    }
}