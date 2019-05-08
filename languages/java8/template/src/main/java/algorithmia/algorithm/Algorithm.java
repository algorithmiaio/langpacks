package algorithmia.algorithm;
import com.algorithmia.algorithmHandler.*;
import com.algorithmia.*;
import java.util.HashMap;



public class Algorithm {

    class AdvancedInput{
        String name;
        Integer age;
    }

    String Apply(AdvancedInput input, HashMap<String, String> context) throws Exception{
            if(context.containsKey("local_file")){
                return "Hello " + input.name + " you are " + input.age +
                        " years old, and your model file is downloaded here " + context.get("local_file");
            }
        return "hello " + input.name+ " you are " + input.age + " years old";
    }
    HashMap<String, String> DownloadModel() throws Exception{
            HashMap<String, String> context = new HashMap<>();
            AlgorithmiaClient client = Algorithmia.client();
            String localFile = client.file("data://demo/collection/testfile.json").getFile().getName();
            context.put("local_file", localFile);
            return context;
    }
    public static void main(String[] args) throws Exception {
        Algorithm defs = new Algorithm();
        AlgorithmHandler algo = new AlgorithmHandler<>(defs, defs::Apply, defs::DownloadModel);
        algo.run();
    }
}
