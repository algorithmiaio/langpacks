using System;
namespace AlgorithmiaPipe
{
    public class AlgoException
    {
        public readonly string message;
        public readonly string stack_trace;
        public readonly string error_type;

        public AlgoException(Exception e)
        {
            {
                message = e.Message;
                stack_trace = e.StackTrace;
                error_type = e.GetType().ToString();
            }
        }
    }
    public class ExceptionResponse
    {
        public readonly AlgoException error;

        public ExceptionResponse(Exception e)
        {
            error = new AlgoException(e);
        }
    }
}