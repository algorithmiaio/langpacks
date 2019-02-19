using System;
using Newtonsoft.Json;
using System.Reflection;
using Pipe;

namespace Pipe
{
    public class Module
    {

        private string DllPath;
        private Type PrimaryClassType;
        private Type AlgoInputType;
        private MethodInfo ApplyMethod;
        
        
        public Module(Config config)
        {
            GetDllPath(config);
            LoadClass(config);
            GetApplyMethod();
            GetAlgoInputType();
        }
        
        
        //TODO: Figure out if this is always the way to capture the dll file of a project. Are there other project formats?
        private void GetDllPath(Config config)
        {
            string algoName = config.Algoname;
            string algoPath = config.Algopath;
            string boilerplate = "bin/Release/netcoreapp2.2";
            string fullpath = $"{algoPath}/{boilerplate}/{algoName}.dll";
            Console.WriteLine(fullpath);
            DllPath = fullpath;
        }
        private void LoadClass(Config config)
        {
            Assembly asm = Assembly.LoadFrom(DllPath);
            PrimaryClassType = asm.GetType($"{config.Algoname}.{config.Algoname}");
        }
        private void GetApplyMethod()
        {
            // getting only public methods, as those are apply methods
            MethodInfo applyMethod = PrimaryClassType.GetMethod("apply");
            if (applyMethod == null || !applyMethod.IsStatic || !applyMethod.IsPublic)
            {
                throw new Exception(
                    "No valid apply methods found. Please ensure that you're algorithm's apply function" +
                    "is public and declared, and your primary class is named after your algorithm.");
            }
            ApplyMethod = applyMethod;
        }


        private void GetAlgoInputType()
        {
            ParameterInfo[] parameters = ApplyMethod.GetParameters();
            if (parameters.Length == 0)
            {
                throw new Exception("the discovered Apply method for your Algorithm did not have any input arguments!");
            }

            ParameterInfo paramInfo = parameters[0];
            AlgoInputType = paramInfo.ParameterType;
        }

        public object AttemptExecute(Request request)
        {
            if (request.ContentType == "json")
            {
                dynamic json = JsonConvert.DeserializeObject(request.Data);
                object deserializedObj = JsonConvert.DeserializeObject(request.Data, AlgoInputType);
                FieldInfo[] fields = AlgoInputType.GetFields();
                foreach (FieldInfo field in fields)
                {
                    dynamic value = json[field.Name];
                    if (value == null)
                    {
                        throw new Exception($"Invalid Json, expected field '{field.Name}' to be defined.");
                    }
                }

                return ApplyMethod.Invoke(null, new[] {deserializedObj});
            }
            else if (request.ContentType == "text")
            {
                return ApplyMethod.Invoke(null, new[] {request.Data});
            }
            else
            {
                throw new Exception($"content_type: '{request.ContentType}' is not implemented!");
            }
        }
    }
}