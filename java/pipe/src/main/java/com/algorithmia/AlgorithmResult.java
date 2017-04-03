package com.algorithmia;

public class AlgorithmResult {
    public static enum ContentType {
        TEXT,
        JSON,
        BINARY
    }

    public final String result;
    public final ContentType contentType;
    public AlgorithmResult(String s, ContentType c) {
        result = s;
        contentType = c;
    }
}