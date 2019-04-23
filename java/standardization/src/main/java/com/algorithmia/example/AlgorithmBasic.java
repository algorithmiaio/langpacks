package com.algorithmia.example;

/**
 * Hello world!
 *
 */
class AlgorithmBasic
{
    static String Apply(String input, String context) throws Exception{
        return "hello ".concat(input);
    }

    public static void main(String[] args) throws Exception{
        AlgorithmHandler algo = new AlgorithmHandler(AlgorithmBasic::Apply);
        algo.run();
    }
}
