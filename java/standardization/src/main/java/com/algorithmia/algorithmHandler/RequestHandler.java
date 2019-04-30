package com.algorithmia.algorithmHandler;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.util.Scanner;
import org.apache.commons.codec.binary.Base64;


final class RequestHandler<INPUT>
{

    private Scanner input;
    private JsonParser parser;

    RequestHandler(){
        this.input = new Scanner(System.in);
        this.parser = new JsonParser();
    }

    private Request<INPUT> CreateRequest(String contentType, JsonElement data) throws Exception {
        if (contentType.equals("json")) {
            if (data.isJsonArray()) {
                JsonArray array = data.getAsJsonArray();
                return new Request<>(contentType, array);
            }else if(data.isJsonPrimitive() || data.isJsonNull()){
                Object[] inputs = {data};
                return new Request<>(contentType, inputs);
            } else {
                return new Request<>(contentType, data);
            }
        } else if (contentType.equals("text")) {
            return new Request<>(contentType, data.getAsString());
        } else if (contentType.equals("binary")) {
            byte[] bytes = Base64.decodeBase64(data.getAsString());
            return new Request<>(contentType, bytes);
        } else {
            throw new Exception("recieved an invalid content_type.");
        }
    }

    Request<INPUT> GetNextRequest() throws Exception{
        if(input.hasNextLine()){
            String line = input.nextLine();
            JsonObject json = parser.parse(line).getAsJsonObject();
            String contentType = json.get("content_type").getAsString();
            JsonElement data = json.get("data");
            return CreateRequest(contentType, data);
        }
        else {
            return null;
        }
    }
}
