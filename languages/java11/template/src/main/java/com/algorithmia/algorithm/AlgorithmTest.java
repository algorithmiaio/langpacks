package com.algorithmia.algorithm;

import junit.framework.Assert;
import junit.framework.TestCase;

public class AlgorithmTest extends TestCase {

    public void testApply() {
        Algorithm algo = new Algorithm();
        String input = "World";
        String result = algo.apply(input);
        String expectedResult = "hello World";
        Assert.assertEquals(result, expectedResult);
    }
}