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
    public static void main(String[] args) throws java.io.FileNotFoundException, java.io.IOException, Throwable {
        String algoname = getAlgorithmName();

        JarRunner runner = null;
        try {
            runner = new JarRunner(algoname, WORKING_DIRECTORY);
        } catch (Throwable t) {
            System.out.println("There was an error loading the algorithm");
            System.out.println(t);
            System.exit(1);
        }

        System.out.println("PIPE_INIT_COMPLETE");
        System.out.flush();

        Scanner input = new Scanner(System.in);
        FileOutputStream fileOutputStream = new FileOutputStream(FIFO_PATH, true);
        PrintStream output = new PrintStream(fileOutputStream, true);
        while (input.hasNextLine()) {
            String line = input.nextLine();
            String serializedJson = null;

            JsonObject json = parser.parse(line).getAsJsonObject();
            String inputContentType = json.get("content_type").getAsString();
            String outputContentType = "text";
            if (inputContentType.equals("text")) {
                Object[] inputArr = {json.get("data")};
                AlgorithmResult result = runner.tryApplies("String-", inputArr);


                serializedJson = getJsonOutput(result);
            } else if (inputContentType.equals("json")) {
                JsonElement data = json.get("data");
                if (data.isJsonArray()) {
                    JsonArray array = data.getAsJsonArray();
                    String methodKey = SignatureUtilities.getMethodKey(array);

                    Object[] inputs = new Object[array.size()];
                    for (int i = 0; i < inputs.length; i++) {
                        inputs[i] = array.get(i);
                    }

                    AlgorithmResult result = runner.tryApplies(methodKey, inputs);

                    serializedJson = getJsonOutput(result);
                } else if (data.isJsonPrimitive() || data.isJsonNull()) {
                    String methodKey = SignatureUtilities.getMethodKeyForElement(data);
                    Object[] inputs = {data};
                    AlgorithmResult result = runner.tryApplies(methodKey, inputs);

                    serializedJson = getJsonOutput(result);
                } else {
                    Object[] inputs = {data};
                    AlgorithmResult result = runner.tryJsonApply(inputs);

                    serializedJson = getJsonOutput(result);
                }
            } else if (inputContentType.equals("binary")) {
                Object[] inputs = {json.get("data")};
                AlgorithmResult result = runner.tryApplies(SignatureUtilities.METHOD_KEY_BYTE_ARRAY, inputs);

                serializedJson = getJsonOutput(result);
            } else {
                throw new IllegalStateException("Wrong content type: " + inputContentType);
            }

            System.out.flush();
            output.println(serializedJson);
            output.flush();
        }
    }

    public static String getJsonOutput(AlgorithmResult algoResult) {
        JsonObject metadata = new JsonObject();

        switch (algoResult.contentType) {
            case TEXT:
                metadata.addProperty("content_type", "text");
                break;
            case JSON:
                metadata.addProperty("content_type", "json");
                break;
            case BINARY:
                metadata.addProperty("content_type", "binary");
                break;
        }

        JsonObject jsonOutput = new JsonObject();
        jsonOutput.addProperty("result", algoResult.result);
        jsonOutput.add("metadata", metadata);

        return jsonOutput.toString();
    }

}
