package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;

import java.util.HashMap;


// This class defines your algorithm.
class Algorithm extends AbstractAlgorithm<String, String>{

    // This class member can be defined as anything you wish, and stores loaded state and mutable state between requests.
     HashMap<String, String> context = new HashMap<>();

    // This apply function defines the primary motive driver of your algorithm. Please ensure that the types defined in
    // your algorithm are the same as those defined in your class implementation above ^

    public String apply(String input){
        // ... = context.get("loaded_model_file")
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

