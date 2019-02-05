using System;
using Newtonsoft.Json;
using System.Reflection;
using Pipe;

namespace Data
{
    class Program
    {

        private static String GetDllPath(Config config)
        {
            
            string algoName = config.Algoname;
            string algoPath = config.Algopath;
            string boilerplate = "bin/Debug/netcoreapp2.2";
            string fullpath = $"{algoPath}/{boilerplate}/{algoName}.dll";
            Console.WriteLine(fullpath);
            return fullpath;

        }

        private static Type LoadClass(string dllPath, string classname)
        {
            Assembly asm = Assembly.LoadFrom(dllPath);
            Type t = asm.GetType(classname);
            return t;
        }
        private static MethodInfo GetApplyMethod(Type t)
        {
            // getting only public methods, as those are apply methods
            MethodInfo applyMethod = t.GetMethod("Apply");
            if (applyMethod == null || !applyMethod.IsStatic || !applyMethod.IsPublic)
            {
                throw new Exception(
                    "No valid apply methods found. Please ensure that you're algorithm's apply function" +
                    "is public and declared, and your primary class is named after your algorithm.");
            }

            
            return applyMethod;
        }


        private static Type GetMethodType(MethodInfo method)
        {
            ParameterInfo[] parameters = method.GetParameters();
            if (parameters.Length == 0)
            {
                throw new Exception("the discovered Apply method for your Algorithm did not have any input arguments!");
            }

            ParameterInfo paramInfo = parameters[0];
            return paramInfo.ParameterType;
        }

        private static object AttemptExecute(MethodInfo applyMethod, Type inputClass, ProcessIO io)
        {
            if (io.ContentType == "json")
            {
                dynamic json = JsonConvert.DeserializeObject(io.Data);
                object deserializedObj = JsonConvert.DeserializeObject(io.Data, inputClass);
                FieldInfo[] fields = inputClass.GetFields();
                foreach (FieldInfo field in fields)
                {
                    dynamic value = json[field.Name];
                    if (value == null)
                    {
                        throw new Exception($"Invalid Json, expected field '{field.Name}' to be defined.");
                    }
                }

                return applyMethod.Invoke(null, new[] {deserializedObj});
            }
            else
            {
                throw new Exception("not implemented!");
            }
        }


        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                throw new Exception(
                    "no algorithm directory argument found. Please provide the path to a valid C# algorithm.");
            }
            string sysPath = args[0];
            Config config = new Config(sysPath);
            ProcessIO io = new ProcessIO();
            string dllPath = GetDllPath(config);
            string className = $"{config.Algoname}.{config.Algoname}";
            Type loaded = LoadClass(dllPath, className);
            MethodInfo applyMethod = GetApplyMethod(loaded);
            Type inputClass = GetMethodType(applyMethod);
            object outputData = AttemptExecute(applyMethod, inputClass, io);
//            string outputJson = JsonConvert.SerializeObject(outputData, Formatting.Indented);
            ProcessIO.WriteToPipe(outputData);
            Console.WriteLine("Algorithm completed, output saved to /tmp/algoout");
        }
    }
}