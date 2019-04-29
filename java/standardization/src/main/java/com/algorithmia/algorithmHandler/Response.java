package com.algorithmia.algorithmHandler;

import com.google.gson.JsonElement;
import org.apache.commons.codec.binary.Base64;

public class Response<T> {
    public MetaData metaData;
    public T result;
    public class MetaData{
        public String content_type;
        MetaData(String contentType){this.content_type = contentType;}
    }
    public Response(T data){
        String contentType;
        Response response;
        if(data == null){
            contentType = "json";
            result = null;
        }
        else if(data instanceof String){
            contentType = "text";
        }
        else if (data instanceof byte[]){
            contentType = "binary";
            String encoded = Base64.encodeBase64String((byte[])data);

        } else {
            contentType = "json";
        }

        metaData = new MetaData(contentType);
        result = data;
    }
}
