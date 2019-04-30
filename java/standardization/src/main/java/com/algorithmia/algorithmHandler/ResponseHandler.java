package com.algorithmia.algorithmHandler;
import java.io.FileOutputStream;
import java.io.PrintStream;

final class ResponseHandler {

    private String FIFOPATH = "/tmp/algoout";

    private PrintStream output;

    ResponseHandler()throws java.io.FileNotFoundException{
        FileOutputStream fileOutputStream = new FileOutputStream(this.FIFOPATH, true);
        output = new PrintStream(fileOutputStream, true);
    }

    <OUTPUT> void writeToPipe(OUTPUT outputObject){
        Response<OUTPUT> response = new Response<>(outputObject);
        String serialized = response.getJsonOutput();
        this.output.println(serialized);
    }

    <ERRORTYPE extends  Throwable> void writeErrorToPipe(ERRORTYPE e){
        SerializableException<ERRORTYPE> exception = new SerializableException<>(e);
        String serialized = exception.getJsonOutput();
        this.output.println(serialized);
    }
}
