package com.algorithmia.algorithm;

import junit.framework.Assert;
import junit.framework.TestCase;

public class AlgorithmTest extends TestCase {

    public void testApply() {
        Algorithm algo = new Algorithm();
        Algorithm.ExampleInput input = algo.new ExampleInput("John", "Doe");
        String result = algo.apply(input);
        String expectedResult = "hello John Doe";
        Assert.assertEquals(result, expectedResult);
    }
}