using System;
using Newtonsoft.Json;
using System.Reflection;
using __ALGO__;

namespace __ALGO__.Pipe
{
    public class Module
    {

        private Type _primaryClassType;
        private Type _algoInputType;
        private MethodInfo _applyMethod;
        
        
        public Module()
        {
            _primaryClassType = typeof(__ALGO__);
            GetApplyMethod();
            GetAlgoInputType();
        }
        

        private void GetApplyMethod()
        {
            // getting only public methods, as those are apply methods
            MethodInfo applyMethod = _primaryClassType.GetMethod("apply");
            if (applyMethod == null || !applyMethod.IsStatic || !applyMethod.IsPublic)
            {
                throw new Exception(
                    "No valid apply methods found. Please ensure that you're algorithm's apply function" +
                    "is public and declared, and your primary class is named after your algorithm.");
            }
            _applyMethod = applyMethod;
        }


        private void GetAlgoInputType()
        {
            ParameterInfo[] parameters = _applyMethod.GetParameters();
            if (parameters.Length == 0)
            {
                throw new Exception("the discovered Apply method for your Algorithm did not have any input arguments!");
            }

            ParameterInfo paramInfo = parameters[0];
            _algoInputType = paramInfo.ParameterType;
        }

        public object AttemptExecute(Request request)
        {
            object[] algorithmArguments;
            if (request.ContentType == "json")
            {
                dynamic json = JsonConvert.DeserializeObject(request.Data);
                object deserializedObj = JsonConvert.DeserializeObject(request.Data, _algoInputType);
                FieldInfo[] fields = _algoInputType.GetFields();
                foreach (FieldInfo field in fields)
                {
                    dynamic value = json[field.Name];
                    if (value == null)
                    {
                        throw new Exception($"Invalid Json, expected field '{field.Name}' to be defined.");
                    }
                }
                

                algorithmArguments = new []{deserializedObj};
            }
            else if (request.ContentType == "text")
            {
                algorithmArguments = new []{request.Data};
            }
            else if (request.ContentType == "binary")
            {
                byte[] binaryGlob = Convert.FromBase64String(request.Data);
                algorithmArguments = new []{binaryGlob};
            }
            else
            {
                throw new Exception($"content_type: '{request.ContentType}' is not implemented!");
            }

            return _applyMethod.Invoke(null, algorithmArguments);
        }
    }
}