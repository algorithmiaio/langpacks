package com.algorithmia.example;

/**
 * Hello world!
 *
 */
class AlgorithmBasic
{
    static String Apply(String input) throws Exception{
        return "hello ".concat(input);
    }
    static AlgorithmPipe Setup() throws Exception{
        AlgorithmPipe algo = new AlgorithmPipe<>(AlgorithmBasic::Apply);
        return algo;
    }
}
