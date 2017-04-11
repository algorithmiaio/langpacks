package com.algorithmia;

import com.google.gson.*;
import java.io.*;
import java.io.FileOutputStream;
import java.io.PrintStream;
import java.util.Scanner;
import java.util.*;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Pipe {
    private static final String WORKING_DIRECTORY = "/opt/algorithm";
    private static final String FIFO_PATH = "/tmp/algoout";
    private static final String CONFIG_FILE_NAME = WORKING_DIRECTORY + "/algorithmia.conf";
    private static final String ALGORITHM_NAME_ENV_VARIABLE = "ALGORITHMIA_ALGORITHM_NAME";
    private static final String ALGORITHM_AUTHOR_ENV_VARIABLE = "ALGORITHMIA_ALGORITHM_AUTHOR_NAME";
    private static final JsonParser parser = new JsonParser();

    private static String getAlgorithmNameFromConfigFile() throws java.io.IOException {
        String contents = new String(Files.readAllBytes(Paths.get(CONFIG_FILE_NAME)));
        JsonObject json = parser.parse(contents).getAsJsonObject();
        return json.get("algoname").getAsString();
    }

    private static String getAlgorithmName() throws java.io.IOException {
        File f = new File(CONFIG_FILE_NAME);
        if (f.exists()) {
            return getAlgorithmNameFromConfigFile();
        }

        return System.getenv().get(ALGORITHM_NAME_ENV_VARIABLE);
    }

    private static String getClassPath() throws java.io.IOException {
        String algoname = getAlgorithmName();

        if (System.getenv().containsKey(ALGORITHM_AUTHOR_ENV_VARIABLE)) {
            String author = System.getenv().get(ALGORITHM_AUTHOR_ENV_VARIABLE);
            return "algorithmia." + author + "." + algoname + "." + algoname;
        }

        return "algorithmia." + algoname + "." + algoname;
    }

    public static void main(String[] args) throws java.io.FileNotFoundException, java.io.IOException {
        String classPath = getClassPath();

        JarRunner runner = null;
        try {
            runner = new JarRunner(classPath, WORKING_DIRECTORY);
        } catch (Throwable t) {
            if (t instanceof ExceptionInInitializerError && t.getCause() != null) {
                t = t.getCause();
            }
            System.out.println("There was an error loading the algorithm");
            StringWriter writer = new StringWriter();
            t.printStackTrace(new PrintWriter(writer));
            System.err.println(writer.toString());
            System.exit(1);
        }

        System.out.println("PIPE_INIT_COMPLETE");
        System.out.flush();

        Scanner input = new Scanner(System.in);

        while (input.hasNextLine()) {
            FileOutputStream fileOutputStream = new FileOutputStream(FIFO_PATH, true);
            PrintStream output = new PrintStream(fileOutputStream, true);
            String line = input.nextLine();
            String serializedJson = null;

            JsonObject json = parser.parse(line).getAsJsonObject();
            String inputContentType = json.get("content_type").getAsString();
            String outputContentType = "text";

            if (runner.hasJsonApply()) {
                JsonElement data = json.get("data");
                serializedJson = tryJsonApply(runner, data).getJsonOutput();
            } else if (inputContentType.equals("text")) {
                Object[] inputArr = {json.get("data")};
                AlgorithmResult result = runner.tryApplies(SignatureUtilities.METHOD_KEY_STRING, inputArr);

                serializedJson = result.getJsonOutput();
            } else if (inputContentType.equals("json")) {
                JsonElement data = json.get("data");
                AlgorithmResult result = null;

                if (data.isJsonArray()) {
                    JsonArray array = data.getAsJsonArray();
                    String methodKey = SignatureUtilities.getMethodKey(array);

                    Object[] inputs = new Object[array.size()];
                    for (int i = 0; i < inputs.length; i++) {
                        inputs[i] = array.get(i);
                    }

                    result = runner.tryApplies(methodKey, inputs);
                } else if (data.isJsonPrimitive() || data.isJsonNull()) {
                    String methodKey = SignatureUtilities.getMethodKeyForElement(data);
                    Object[] inputs = {data};
                    result = runner.tryApplies(methodKey, inputs);

                } else {
                    result = tryJsonApply(runner, data);
                }

                // We failed to pass the args from the list as paramaters. Lets run it as json now.
                if (result.error != AlgorithmResult.ErrorType.NONE) {
                    AlgorithmResult jsonAttempt = tryJsonApply(runner, data);
                    // We only want to use this result if it is "better" than what we had before.
                    if ((jsonAttempt.error == AlgorithmResult.ErrorType.NONE || jsonAttempt.error == AlgorithmResult.ErrorType.RUNNING) && result.error == AlgorithmResult.ErrorType.INVOCATION) {
                        result = jsonAttempt;
                    }
                }

                serializedJson = result.getJsonOutput();
            } else if (inputContentType.equals("binary")) {
                Object[] inputs = {json.get("data")};
                AlgorithmResult result = runner.tryApplies(SignatureUtilities.METHOD_KEY_BYTE_ARRAY, inputs);

                serializedJson = result.getJsonOutput();
            } else {
                throw new IllegalStateException("Wrong content type: " + inputContentType);
            }

            System.out.flush();
            output.println(serializedJson);
            output.flush();
            fileOutputStream.flush();

            output.close();
            fileOutputStream.close();
        }
    }

    private static AlgorithmResult tryJsonApply(JarRunner runner, JsonElement data) {
        Object[] inputs = {data};
        return runner.tryJsonApply(inputs);
    }
}
