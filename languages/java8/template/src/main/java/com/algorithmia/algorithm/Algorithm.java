package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;

class Algorithm implements AlgorithmInterface<String, String>{


    public String apply(String input){
        return "hello ".concat(input);
    }

    public void load(){
        System.out.println("Implement this for loading functionality");
    }

    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new Handler<>(algorithm);
        algo.serve();
    }
}

