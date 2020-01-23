package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;
import com.google.gson.JsonObject;
import java.lang.reflect.Field;
import java.util.Map;


// This class defines your algorithm.
class Algorithm extends AbstractAlgorithm<Object, String>{

    private Model model;
    // This class defines the input to your algorithm, the algorithmia platform will attempt to deserialize JSON into this type.


    private static void injectEnvironmentVariable(String key, String value)
            throws Exception {

        Class<?> processEnvironment = Class.forName("java.lang.ProcessEnvironment");

        Field unmodifiableMapField = getAccessibleField(processEnvironment, "theUnmodifiableEnvironment");
        Object unmodifiableMap = unmodifiableMapField.get(null);
        injectIntoUnmodifiableMap(key, value, unmodifiableMap);

        Field mapField = getAccessibleField(processEnvironment, "theEnvironment");
        Map<String, String> map = (Map<String, String>) mapField.get(null);
        map.put(key, value);
    }

    private static Field getAccessibleField(Class<?> clazz, String fieldName)
            throws NoSuchFieldException {

        Field field = clazz.getDeclaredField(fieldName);
        field.setAccessible(true);
        return field;
    }

    private static void injectIntoUnmodifiableMap(String key, String value, Object map)
            throws ReflectiveOperationException {

        Class unmodifiableMap = Class.forName("java.util.Collections$UnmodifiableMap");
        Field field = getAccessibleField(unmodifiableMap, "m");
        Object obj = field.get(map);
        ((Map<String, String>) obj).put(key, value);
    }


    // This apply function defines the primary motive driver of your algorithm. Please ensure that the types defined in
    // your algorithm are the same as those defined in as generic variables in your concrete class defined above.

    public String apply(Object input){
        JsonObject prediction = model.predict((Map) input);
        return prediction.toString();
    }

    @Override
    public void load() throws RuntimeException{
        try{
            AlgorithmiaClient client = Algorithmia.client("simaZLholIVq0jNYeqfxRAFKaIw1", "https://api.test.algorithmia.com");
            String modelPath = client.file("data://zeryx/driverless/pipeline.mojo").getFile().getAbsolutePath();
//            String modelPath = "/tmp/pipeline.mojo";
            String licensePath = client.file("data://zeryx/driverless/license.sig").getFile().getAbsolutePath();
            System.out.println(modelPath);
            System.out.println(licensePath);
            injectEnvironmentVariable("DRIVERLESS_AI_LICENSE_FILE", licensePath);
            model = new Model(modelPath);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new Handler<>(algorithm);
        algo.serve();
    }
}

