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
                        throw new AlgorithmiaRunnerException("can only have a single apply() method if specifying AcceptsJson");
                    }
                }

                if (a.toString().equals("@com.algorithmia.algo.ReturnsJson()")) {
                    returnJsonMethods.add(m);
                }
            }
        }

        if (jsonApplyMethodData != null && applyMethods.size() != 2) {
            throw new AlgorithmiaRunnerException("can only have a single apply() method if specifying AcceptsJson");
        }
    }

    public AlgorithmResult tryApplies(String methodKey, Object[] inputObject) {
        ErrorPair errorPair = new ErrorPair();

        try {
            return tryAppliesInternal(methodKey, inputObject);
        } catch (Exception e) {
            errorPair.registerNewException(e);
        }

        try {
            return tryAppliesInternal(SignatureUtilities.getGenericKey(inputObject.length), inputObject);
        } catch (Exception e) {
            errorPair.registerNewException(e);
        }

        return errorPair.getResult();
    }

    @SuppressWarnings("unchecked")
    private AlgorithmResult tryAppliesInternal(String methodKey, Object[] inputObject) throws Exception {
        if (!applyMethods.containsKey(methodKey)) {
            throw new AlgorithmiaRunnerException("no apply method matches input signature");
        }

        Exception exception = null;
        for (MethodData mcp : applyMethods.get(methodKey)) {
            Object[] convertedInputs = new Object[inputObject.length];

            try {
                for (int i = 0; i < convertedInputs.length; i++) {
                    convertedInputs[i] = mcp.conversions[i].apply(inputObject[i]);
                }
                return applyInput(mcp.method, convertedInputs);
            } catch (Exception t) {
                exception = t;
            }
        }

        if (exception != null) {
            throw exception;
        }

        throw new AlgorithmiaRunnerException("no apply method was successfully applied to the input");
    }

    public AlgorithmResult tryJsonApply(Object[] inputObject) {
        ErrorPair errorPair = new ErrorPair();

        try {
            return tryJsonApplyInternal(inputObject);
        } catch (Exception e) {
            errorPair.registerNewException(e);
        }

        try {
            return tryAppliesInternal(SignatureUtilities.getGenericKey(inputObject.length), inputObject);
        } catch (Exception e) {
            errorPair.registerNewException(e);
        }

        return errorPair.getResult();
    }

    @SuppressWarnings("unchecked")
    public AlgorithmResult tryJsonApplyInternal(Object[] inputObject) throws Exception {
        if (jsonApplyMethodData == null) {
            return tryAppliesInternal(SignatureUtilities.METHOD_KEY_OBJECT, inputObject);
        }

        Object[] convertedInputs = new Object[inputObject.length];

        for (int i = 0; i < convertedInputs.length; i++) {
            convertedInputs[i] = jsonApplyMethodData.conversions[i].apply(inputObject[i]);
        }
        return applyInput(jsonApplyMethodData.method, convertedInputs);
    }

    private AlgorithmResult applyInput(Method applyMethod, Object[] inputObject) throws Exception {
        Object output = applyMethod.invoke(instance, inputObject);

        if (output == null) {
            return new AlgorithmResult((JsonElement)null);
        } else if (output instanceof String) {
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
                throw new AlgorithmiaRunnerException("failed to process algorithm output", e);
            }
        } else if (output instanceof byte[]) {
            return new AlgorithmResult(Base64.getEncoder().encodeToString((byte[])output), AlgorithmResult.ContentType.BINARY);
        } else {
            try {
                return new AlgorithmResult(SignatureUtilities.gson.toJsonTree(output, applyMethod.getGenericReturnType()));
            } catch (Throwable e) {
                throw new AlgorithmiaRunnerException("failed to parse algorithm output", e);
            }
        }
    }

    public boolean hasJsonApply() {
        return jsonApplyMethodData != null;
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
            throw new AlgorithmiaRunnerException("Algorithm class not found. Name must match: " + classPath + "\nTo Fix: Double check both package-name and class-name");
        } catch (Throwable e) {
            throw e;
        }
    }

    private class AlgorithmiaRunnerException extends Exception {
        public AlgorithmiaRunnerException() { super(); }
        public AlgorithmiaRunnerException(String message) { super(message); }
        public AlgorithmiaRunnerException(String message, Throwable cause) { super(message, cause); }
        public AlgorithmiaRunnerException(Throwable cause) { super(cause); }
    }


    private class ErrorPair {
        public Exception callError;
        public Exception algorithmError;

        public void registerNewException(Exception e) {
            if (e instanceof AlgorithmiaRunnerException) {
                if (callError == null) {
                    callError = e;
                }
            } else if (e instanceof ClassCastException || e instanceof IllegalArgumentException || e instanceof IllegalAccessException) {
                if (callError == null) {
                    callError = new AlgorithmiaRunnerException("failed to invoke algorithm", e);
                }
            } else if (e instanceof InvocationTargetException) {
                if (callError == null) {
                    if (e.getCause() == null) {
                        callError = new AlgorithmiaRunnerException("failed to invoke algorithm", e);
                    } else {
                        callError = new AlgorithmiaRunnerException(e.getCause());
                    }
                }
            } else {
                if (algorithmError == null) {
                    algorithmError = e;
                }
            }
        }

        public AlgorithmResult getResult() {
            AlgorithmResult.ErrorType errorType = algorithmError != null ? AlgorithmResult.ErrorType.RUNNING : AlgorithmResult.ErrorType.INVOCATION;
            Exception e = algorithmError != null ? algorithmError : callError;

            StringWriter writer = new StringWriter();
            e.printStackTrace(new PrintWriter(writer));

            JsonObject inner = new JsonObject();
            String message = (e.getMessage() == null) ? e.toString() : e.getMessage().toString();
            inner.addProperty("message", message);
            inner.addProperty("stacktrace", writer.toString());
            inner.addProperty("error_type", "AlgorithmError");

            JsonObject outer = new JsonObject();
            outer.add("error", inner);
            return new AlgorithmResult(outer, errorType);
        }
    }
}