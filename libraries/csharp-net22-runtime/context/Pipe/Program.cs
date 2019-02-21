using System;

namespace Pipe
{
    class Program
    {
        static int Main(string[] args)
        {
            string sysPath;
            if (args.Length == 0)
            {
                sysPath = ".";
            }
            else
            {
                sysPath = args[0];
            }
            Config config = new Config(sysPath);
            Module algoModule;
            try
            {
                algoModule = new Module(config);
            }
            catch (Exception e)
            {
                ExceptionResponse response = new ExceptionResponse(e);
                Write.WriteJsonToPipe(response);
                return -1;
            }
            Console.Out.WriteLine("PIPE_INIT_COMPLETE");
            Console.Out.Flush();
            string readLine;
            while ((readLine = Console.In.ReadLine()) != null)
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
            return 0;
        }
    }
}