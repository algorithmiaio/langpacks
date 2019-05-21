package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;


// This class defines your algorithm.
class Algorithm implements AlgorithmInterface<String, String>{


    public String apply(String input){
        return "hello ".concat(input);
    }

    // For more information around how loading works, please check out the algorithm docs in algorithmia.com/developers/
    public void load(){
        System.out.println("Implement this for loading functionality");
    }

    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new Handler<>(algorithm);
        algo.serve();
    }
}

