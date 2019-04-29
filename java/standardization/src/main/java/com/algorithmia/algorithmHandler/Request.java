package com.algorithmia.algorithmHandler;

public class Request<T>{
    public String content_type;
    public T data;

    private T CheckCompatability(Object req) throws Exception{
        T type = null;
        try{
            T algoInput = (T)req;
            return algoInput;
        } catch (Exception e) {
            throw new Exception("request " + req.getClass() + " is not valid for apply of type " + type.getClass());
        }
    }

    public Request(String content_type, Object data) throws Exception{
        T formatted = CheckCompatability(data);

        this.content_type = content_type;
        this.data = formatted;
    }
}
