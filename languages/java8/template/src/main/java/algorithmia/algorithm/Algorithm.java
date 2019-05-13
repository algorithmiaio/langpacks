package algorithmia.algorithm;

import com.algorithmia.algorithmHandler.*;
import com.algorithmia.*;

class Algorithm {
    String apply(String input) throws Exception {
        return "hello ".concat(input);
    }

    public static void main(String[] args) throws Exception {
        AlgorithmBasic defs = new AlgorithmBasic();
        AlgorithmHandler algo = new AlgorithmHandler<>(defs, defs::apply);
        algo.run();
    }
}


/**
 * This class below describes a more advanced template.
 * If your algorithm needs key data or model files at runtime, you don't want to be downloading that every time your algorithm gets called.
 * In this situation, you will want to designate a `Loading` function (like DownloadModel below), and provide that method to the AlgorithmHandler constructor.
 * For more information, please refer to the advanced user guide in https://docs.algorithmia.com
 */

//public class AlgorithmAdvanced {
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
//        AlgorithmiaClient client = Algorithmia.client();
//        String localFile = client.file("data://demo/collection/testfile.json").getFile().getName();
//        context.put("local_file", localFile);
//        return context;
//    }
//
//    public static void main(String[] args) throws Exception {
//        AlgorithmAdvanced defs = new AlgorithmAdvanced();
//        AlgorithmHandler algo = new AlgorithmHandler<>(defs, defs::apply, defs::downloadModel);
//        algo.run();
//    }
//}
