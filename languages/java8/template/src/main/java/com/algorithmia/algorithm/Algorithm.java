package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;

class Algorithm {
    public String apply(String input) throws RuntimeException {
        return "hello ".concat(input);
    }

    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new Handler<>(algorithm.getClass(), algorithm::apply);
        algo.serve();
    }
}


/**
 * This class below describes a more advanced template.
 * If your algorithm needs key data or model files at runtime, you don't want to be downloading that every time your algorithm gets called.
 * In this situation, you will want to designate a `Loading` function (like DownloadModel below), and provide that method to the AlgorithmHandler constructor.
 * For more information, please refer to the advanced user guide in https://docs.algorithmia.com
 */

//public class Algorithm {
//
//    class AdvancedInput {
//        String name;
//        Integer age;
//    }
//
//    String apply(AdvancedInput input, HashMap<String, String> context) throws Exception {
//        if (context.containsKey("local_file")) {
//            return "Hello " + input.name + " you are " + input.age +
//                    " years old, and your model file is downloaded here " + context.get("local_file");
//        }
//        return "hello " + input.name + " you are " + input.age + " years old";
//    }
//
//    HashMap<String, String> downloadModel() throws Exception {
//        HashMap<String, String> context = new HashMap<>();
//        context["local_file"] = "/tmp/some_example.json"
//        return context;
//    }
//
//    public static void main(String[] args) throws Exception {
//        Algorithm algorithm = new Algorithm();
//        AlgorithmHandler algo = new AlgorithmHandler<>(algorithm.getClass(), algorithm::apply, algorithm::downloadModel);
//        algo.run();
//    }
//}
