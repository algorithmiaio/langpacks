using System;
using System.Data;
using System.IO;
using Newtonsoft.Json;
using System.Reflection;
namespace Pipe
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
//            MethodInfo[] allMethods = t.GetMethods(BindingFlags.Public|BindingFlags.DeclaredOnly);
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

        private static object AttemptExecute(MethodInfo applyMethod, Type inputClass,  string dataPath)
        {
            using (StreamReader r = new StreamReader(dataPath))
            {
                string jstring = r.ReadToEnd();
                try
                {
                    dynamic json = JsonConvert.DeserializeObject(jstring);
                    object deserializedObj = JsonConvert.DeserializeObject(jstring, inputClass);
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
                catch (Exception e)
                {
                    Console.WriteLine(e);
                    throw new Exception($"input {jstring} did not conform to expected input class type: ${inputClass.Name}\n {e.Message}");
                }
            }
        }

        private static object AttemptExecute(MethodInfo applyMethod, string dataPath)
        {
            
            
            return new object();
        }
        
        
        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                throw new Exception(
                    "no algorithm directory argument found. Please provide the path to a valid C# algorithm.");
            }
            string sysPath = args[0];
            string inputPath = args[1];
            Config config = new Config(sysPath);
            string dllPath = GetDllPath(config);
            string className = $"{config.Algoname}.{config.Algoname}";
            Type loaded = LoadClass(dllPath, className);
            MethodInfo applyMethod = GetApplyMethod(loaded);
            Type inputClass = GetMethodType(applyMethod);
            object outputData = AttemptExecute(applyMethod, inputClass, inputPath);
            string outputJson = JsonConvert.SerializeObject(outputData, Formatting.Indented);
            Console.WriteLine(outputJson);
        }
    }
}