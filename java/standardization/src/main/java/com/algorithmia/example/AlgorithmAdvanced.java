package com.algorithmia.example;
import com.algorithmia.*;
import java.util.HashMap;

public class AlgorithmAdvanced {
        static String Apply(String input) throws Exception{
        return "hello ".concat(input);
    }
    static HashMap<String, Object> DownloadModel() throws Exception{
            HashMap<String, Object> context = new HashMap<>();
            AlgorithmiaClient client = Algorithmia.client(System.getenv("ALGORITHMIA_API_KEY"));
            String localFile = client.file("data://.my/collection/testfile.json").getFile().getName();
            context.put("local_file", localFile);
            return context;
    }

        static AlgorithmPipe Setup() throws Exception {
            AlgorithmPipe<String, String> algo = new AlgorithmPipe<>();
            algo.setApply(AlgorithmBasic::Apply);
            algo.setLoad(AlgorithmAdvanced::DownloadModel);
            return algo;
        }

}
