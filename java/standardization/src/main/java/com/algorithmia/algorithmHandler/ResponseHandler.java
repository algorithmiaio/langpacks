package com.algorithmia.algorithmHandler;
import java.io.FileOutputStream;
import java.io.PrintStream;
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import org.apache.commons.codec.binary.Base64;

final class ResponseHandler<OUTPUT> {

    private String FIFOPATH = "/tmp/algoout";

    private PrintStream output;

    ResponseHandler()throws java.io.FileNotFoundException{
        FileOutputStream fileOutputStream = new FileOutputStream(this.FIFOPATH, true);
        this.output = new PrintStream(fileOutputStream, true);
    }

    public void writeToPipe(OUTPUT outputObject){
    }
}
