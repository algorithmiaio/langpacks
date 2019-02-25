using System;
namespace __ALGO__.Pipe
{
    class Pipe
    {
        static int Main()
        {
            Module algoModule;
            try
            {
                algoModule = new Module();
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
                    if (result != null)
                    {
                        response = new Response(result, "json");
                    }
                    else
                    {
                        response = new ExceptionResponse(new Exception("the response from the algorithm was 'null'. \n" +
                                                                       "Algorithms are not allowed to return 'null'."));
                    }
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