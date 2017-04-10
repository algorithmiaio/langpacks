package com.algorithmia;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

public class AlgorithmResult {
    public static enum ContentType {
        TEXT,
        JSON,
        BINARY
    }

    public static enum ErrorType {
        NONE,
        INVOCATION,
        RUNNING
    }

    private final String resultString;
    private final JsonElement resultJson;
    private final ContentType contentType;
    public final ErrorType error;

    public AlgorithmResult(String s, ContentType c) {
        resultString = s;
        resultJson = null;
        contentType = c;
        error = ErrorType.NONE;
    }

    public AlgorithmResult(JsonElement j) {
        this(j, ErrorType.NONE);
    }

    public AlgorithmResult(JsonElement j, ErrorType e) {
        resultJson = j;
        resultString = null;
        contentType = ContentType.JSON;
        error = e;
    }

    public String getJsonOutput() {
        if (error != ErrorType.NONE) {
            return resultJson.toString();
        }

        JsonObject metadata = new JsonObject();

        switch (contentType) {
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
        jsonOutput.add("metadata", metadata);

        if (resultString != null) {
            jsonOutput.addProperty("result", resultString);
        } else {
            jsonOutput.add("result", resultJson);
        }

        return jsonOutput.toString();
    }
}