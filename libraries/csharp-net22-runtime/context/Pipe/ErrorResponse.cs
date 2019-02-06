using System;
namespace Pipe
{
    public class ExceptionResponse
    {
        public string message;
        public string stack_trace;
        public string error_type;

        public ExceptionResponse(Exception e)
        {
            message = e.Message;
            stack_trace = e.StackTrace;
            error_type = e.GetType().ToString();
        }
    }
}