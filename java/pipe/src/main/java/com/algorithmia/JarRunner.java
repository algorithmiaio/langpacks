package com.algorithmia;

import com.google.gson.*;
import java.lang.reflect.*;
import java.lang.annotation.*;
import java.io.*;
import java.util.Collection;
import java.util.*;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Paths;
import org.apache.commons.io.FileUtils;
import org.apache.commons.io.IOUtils;

public class JarRunner {
    private Map<String, Queue<MethodData>> applyMethods;
    private Set<Method> returnJsonMethods;
    private MethodData jsonApplyMethodData;
    private Object instance;

    private void addToMethods(MethodData methodData) {
        String key = methodData.key;
        if (!applyMethods.containsKey(key)) {
            applyMethods.put(key, new PriorityQueue<MethodData>());
        }
        applyMethods.get(key).add(methodData);
    }

    public JarRunner(String classPath, String workingDirectory) throws Exception {
        Class<?> algoClass = loadAlgorithm(classPath, workingDirectory);
        instance = algoClass.newInstance();
        applyMethods = new HashMap<String, Queue<MethodData>>();
        returnJsonMethods = new HashSet<Method>();

        for (Method m : algoClass.getMethods()) {
            if (!m.getName().equals("apply")) {
                continue;
            }

            MethodData genericMethodData = SignatureUtilities.getGenericMethodData(m);
            addToMethods(genericMethodData);

            MethodData methodData = SignatureUtilities.getMethodData(m);
            addToMethods(methodData);

            for (Annotation a : m.getAnnotations()) {
                if (a.toString().equals("@com.algorithmia.algo.AcceptsJson()")) {
                    if (jsonApplyMethodData == null) {
                        jsonApplyMethodData = methodData;
                    } else {
                        throw new Exception("can only have a single apply() method if specifying AcceptsJson");
                    }
                }

                if (a.toString().equals("@com.algorithmia.algo.ReturnsJson()")) {
                    returnJsonMethods.add(m);
                }
            }
        }

        if (jsonApplyMethodData != null && applyMethods.size() != 1) {
            throw new Exception("can only have a single apply() method if specifying AcceptsJson");
        }
    }

    public AlgorithmResult tryApplies(String methodKey, Object[] inputObject) throws Exception {
        try {
            return tryAppliesInternal(methodKey, inputObject);
        } catch (Throwable e) {
            return tryAppliesInternal(SignatureUtilities.getGenericKey(inputObject.length), inputObject);
        }
    }

    @SuppressWarnings("unchecked")
    private AlgorithmResult tryAppliesInternal(String methodKey, Object[] inputObject) throws Exception {
        if (!applyMethods.containsKey(methodKey)) {
            throw new Exception("no apply method matches input signature");
        }


        for (MethodData mcp : applyMethods.get(methodKey)) {
            Object[] convertedInputs = new Object[inputObject.length];

            try {
                for (int i = 0; i < convertedInputs.length; i++) {
                    convertedInputs[i] = mcp.conversions[i].apply(inputObject[i]);
                }
                return applyInput(mcp.method, convertedInputs);
            } catch (Throwable t) {
                // Ignore exceptions
            }
        }

        throw new Exception("no apply method was successfully applied to the input + " + inputObject[0].getClass().getName());
    }

    @SuppressWarnings("unchecked")
    public AlgorithmResult tryJsonApply(Object[] inputObject) throws Exception {
        if (jsonApplyMethodData == null) {
            throw new IllegalStateException("There is no json apply method");
        }

        Object[] convertedInputs = new Object[inputObject.length];

        try {
            for (int i = 0; i < convertedInputs.length; i++) {
                convertedInputs[i] = jsonApplyMethodData.conversions[i].apply(inputObject[i]);
            }
            return applyInput(jsonApplyMethodData.method, convertedInputs);
        } catch (Throwable t) {
            // Ignore exceptions
        }

        throw new Exception("json apply failed");
    }

    private AlgorithmResult applyInput(Method applyMethod, Object[] inputObject) throws Exception {
        Object output = null;
        try {
            output = applyMethod.invoke(instance, inputObject);
        } catch (ClassCastException e) {
            throw new Exception("failed to invoke algorithm", e);
        } catch (IllegalArgumentException e) {
            throw new Exception("failed to invoke algorithm", e);
        } catch (IllegalAccessException e) {
            throw new Exception("failed to invoke algorithm", e);
        } catch (InvocationTargetException e) {
            if (e.getCause() == null) {
                throw new Exception("failed to invoke algorithm", e);
            } else {
                throw new Exception(e.getCause());
            }
        }

        if (output == null) {
            return null;
        }

        if (output instanceof String) {
            if (returnJsonMethods.contains(applyMethod)) {
                return new AlgorithmResult((String)output, AlgorithmResult.ContentType.JSON);
            } else {
                return new AlgorithmResult((String)output, AlgorithmResult.ContentType.TEXT);
            }
        } else if (output instanceof File) {
            try {
                File file = (File)output;
                BufferedInputStream bis = new BufferedInputStream(new FileInputStream(file.getAbsolutePath()));
                String out = Base64.getEncoder().encodeToString(IOUtils.toByteArray(bis));
                file.delete();
                return new AlgorithmResult(out, AlgorithmResult.ContentType.BINARY);
            } catch (Throwable e) {
                throw new Exception("failed to process algorithm output", e);
            }
        } else if (output instanceof byte[]) {
            return new AlgorithmResult(Base64.getEncoder().encodeToString((byte[])output), AlgorithmResult.ContentType.BINARY);
        } else {
            try {
                return new AlgorithmResult(SignatureUtilities.gson.toJson(output, applyMethod.getGenericReturnType()), AlgorithmResult.ContentType.JSON);
            } catch (Throwable e) {
                throw new Exception("failed to parse algorithm output", e);
            }
        }
    }

    /**
     * Resolve algorithm with ivy and load JARs into a ClassLoader
     */
    private ClassLoader loadJars(String workingDirectory) {
        // Load JARs
        String[] extensions = {"jar"};
        Collection<File> jarFiles = FileUtils.listFiles(new File(workingDirectory), extensions, true); // Recursively find jars
        List<URL> jarUrls = new ArrayList<URL>(jarFiles.size());


        for (File jar : jarFiles) {
            try {
                jarUrls.add(new URL("file:" + jar));
            } catch (Throwable t) {
                // Do nothing
            }
        }

        URL[] jarUrlArray = new URL[jarUrls.size()];
        jarUrlArray = jarUrls.toArray(jarUrlArray);

        // Return ClassLoader
        return URLClassLoader.newInstance(
            jarUrlArray,
            getClass().getClassLoader()
        );
    }

    private Class<?> loadAlgorithm(String classPath, String workingDirectory) throws Exception {
        try {
            ClassLoader loader = loadJars(workingDirectory);
            return loader.loadClass(classPath);
        } catch (ClassNotFoundException e) {
            throw new Exception("Algorithm class not found. Name must match: " + classPath + "\nTo Fix: Double check both package-name and class-name");
        } catch (Throwable e) {
            throw e;
        }
    }
}