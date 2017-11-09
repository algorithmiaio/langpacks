package algorithmia.__ALGO__;

import junit.framework.Test;
import junit.framework.TestCase;
import junit.framework.TestSuite;

/**
 * Unit test for __ALGO__ algorithm
 */
public class __ALGO__Test extends TestCase {
    /**
     * Create the test case
     *
     * @param testName name of the test case
     */
    public __ALGO__Test(String testName) {
        super(testName);
    }

    /**
     * @return the suite of tests being tested
     */
    public static Test suite() {
        return new TestSuite(__ALGO__Test.class);
    }

    /**
     * Basic test
     */
    public void test__ALGO__() throws Exception {
        __ALGO__ algorithm = new __ALGO__();
        assertEquals(algorithm.apply("Bob"), "Hello Bob");
    }
}
