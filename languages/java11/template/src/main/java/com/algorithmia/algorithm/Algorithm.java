package com.algorithmia.algorithm;

import com.algorithmia.development.*;
import com.algorithmia.*;

import java.util.HashMap;



class Algorithm extends AbstractAlgorithm<String, String>{

    // This class defines the input to your algorithm, the algorithmia platform will attempt to deserialize JSON into this type.


    // This apply function defines the primary motive driver of your algorithm. Please ensure that the types defined in
    // your algorithm are the same as those defined in as generic variables in your concrete class defined above.

    public String apply(String input){
            return "hello " + input;
    }


    public static void main(String[] args) {
        Algorithm algorithm = new Algorithm();
        Handler algo = new ADK<>(algorithm);
        algo.init();
    }
}


// This class defines your algorithm.


// below is an advanced example that contains a custom input json schema, with a simple string output
// class Algorithm extends AbstractAlgorithm<Algorithm.ExampleInput, String>{
//
//
//     class ExampleInput {
//         //If you flag a field with the @Required annotation, we will only validate the deserialization operation if the field is present.
//         @Required String first_name;
//         //If the @Required annotation is not present, then we will use the default / null value for that type if the field isn't present, consider it "optional".
//         String last_name;
//         ExampleInput(String first_name, String last_name){
//             this.first_name = first_name;
//             this.last_name = last_name;
//         }
//     }
//
//
//     // This apply function defines the primary motive driver of your algorithm. Please ensure that the types defined in
//     // your algorithm are the same as those defined in as generic variables in your concrete class defined above.
//
//     public String apply(ExampleInput input){
//         if(input.last_name != null){
//             return "hello " + input.first_name + " " + input.last_name;
//         }
//         else {
//             return "hello " + input.first_name;
//         }
//     }
//
//
//     public static void main(String[] args) {
//         Algorithm algorithm = new Algorithm();
//         ADK algo = new ADK<>(algorithm);
//         algo.init();
//     }
// }

