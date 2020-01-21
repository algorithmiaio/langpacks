package com.algorithmia.algorithm;

import junit.framework.Assert;
import junit.framework.TestCase;

import java.util.HashMap;

public class AlgorithmTest extends TestCase {

    public void testApply() {
        Algorithm algo = new Algorithm();
        Object input = new HashMap<String, String>();
        String result = algo.apply(input);
        String expectedResult = "hello John Doe";
        Assert.assertEquals(result, expectedResult);
    }
}