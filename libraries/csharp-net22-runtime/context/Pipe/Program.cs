using System;
using System.IO;
using System.Collections.Generic;
using System.ComponentModel;
using System.Reflection;
namespace Pipe
{
    class Program
    {

        private static String GetDllPath(Config config)
        {
            
            string algoname = config.Algoname;
            string pwd = Directory.GetCurrentDirectory();
            string fullpath = $"{pwd}/{algoname}.dll";
            return fullpath;

        }

        private static List<MethodInfo> GetApplyMethods(string dllPath, string classname)
        {
            List < MethodInfo > applyMethods = new List<MethodInfo>();
            Assembly asm = Assembly.LoadFrom(dllPath);
            Type t = asm.GetType(classname);
            // getting only public methods, as those are apply methods
            MethodInfo[] allMethods = t.GetMethods(BindingFlags.Public|BindingFlags.Instance|BindingFlags.DeclaredOnly);
            for (int i = 0; i < allMethods.Length; i++)
            {
                MethodInfo chk = allMethods[i];
                if (chk.Name == "apply" | chk.Name == "Apply")
                {
                    applyMethods.Add(chk);
                }
                
            }

            if (applyMethods.Count == 0)
            {
                throw new Exception(
                    "No valid apply methods found. Please ensure that you're algorithm's apply function" +
                    "is public and declared, and your primary class is named after your algorithm.");
            }

            return applyMethods;
        }
        
        
        static void Main(string[] args)
        {
            Config config = new Config();
            string dllPath = GetDllPath(config);
            string className = $"{config.Algoname}.{config.Algoname}";
            List<MethodInfo> applyMethods = GetApplyMethods(dllPath, className);
            
        }
    }
}