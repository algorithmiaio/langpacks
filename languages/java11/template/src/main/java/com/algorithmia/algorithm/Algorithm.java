package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;

import java.util.HashMap;


// This class defines your algorithm.
class Algorithm extends AbstractAlgorithm<String, String>{

//     HashMap<String, String> context = new HashMap<>();
    

    public String apply(String input){
        return "hello ".concat(input);
    }

    // For more information around how loading works, please check out the algorithm docs in algorithmia.com/developers/
    //<--- Implement this for loading functionality -->
    public void load(){
        // context.put("loaded_model_file") = ...
    }

    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new Handler<>(algorithm);
        algo.serve();
    }
}

