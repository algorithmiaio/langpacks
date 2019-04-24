using System;
using System.Collections.Generic;
using System.Reflection;
using AlgorithmiaPipe;
using Newtonsoft.Json;

namespace Algo.Devtools
{


    public class AlgorithmHandler<I1, I2, O>
    {

        public delegate O ApplyMethod1(I1 input);
        public delegate O ApplyMethod2(I1 input, I2 context);

        public delegate I2 LoadMethod();

        private I2 _context = default;
        private LoadMethod _loadMethod = () => { return default;};
        private ApplyMethod1 _applyMethod1;
        private ApplyMethod2 _applyMethod2;
        public AlgorithmHandler(ApplyMethod1 func)
        {
            _applyMethod1 = func;
        }

        public AlgorithmHandler(ApplyMethod2 func)
        {
            _applyMethod2 = func;
        }


        public void SetLoadFunction(LoadMethod func)
        {
            _loadMethod = func;
        }




        private void Load()
        {
            _context = _loadMethod();
            Console.Out.WriteLine("PIPE_INIT_COMPLETE");
            Console.Out.Flush();
        }

        private O Invoke(I1 input)
        {
            O output;
            if (_applyMethod2 != null)
            {
                output = _applyMethod2.Invoke(input, _context);
            }
            else
            {
                output =  _applyMethod1.Invoke(input);
            }

            return output;
        }


        private O AttemptExecute(Request request)
        {
            dynamic algorithmArguments = ValidateInput(request);
            try
            {
                return Invoke(algorithmArguments);
            }
            catch (TargetInvocationException e)
            {
                throw e.InnerException;
            }
        }

        private void FaaSExecute()
        {
            string readLine;
            while ((readLine = Console.In.ReadLine()) != null)
            {
                Request request = new Request(readLine);
                object response = null;
                try
                {
                    object result = AttemptExecute(request);
                    if (result != null)
                    {
                        response = new Response(result);
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
        }

        private object ValidateInput(Request request)
        {
            object algorithmArguments;
            Type algorithmInputType = typeof(I1);
            if (request.ContentType == "json")
            {
                algorithmArguments = JsonConvert.DeserializeObject(request.Data, algorithmInputType);
            }
            else if (request.ContentType == "text")
            {
                algorithmArguments = request.Data;
            }
            else if (request.ContentType == "binary")
            {
                byte[] binaryGlob = Convert.FromBase64String(request.Data);
                algorithmArguments = new {binaryGlob};
            }
            else
                throw new Exception($"content_type: '{request.ContentType}' is not implemented!");

            return algorithmArguments;
        }

        public void Run()
        {
            Load();
            FaaSExecute();
        }
    }
}