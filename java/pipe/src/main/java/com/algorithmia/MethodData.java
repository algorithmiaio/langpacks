package com.algorithmia;

import java.lang.reflect.Method;
import java.util.function.Function;

public class MethodData implements Comparable<MethodData> {
    public Method method;
    public String key;
    public Function[] conversions;

    public MethodData(Method m, String k, Function[] c) {
        method = m;
        key = k;
        conversions = c;
    }

    public int compareTo(MethodData o) {
        if (o.conversions.length != this.conversions.length) {
            return o.conversions.length - this.conversions.length;
        }

        for (int i = 0; i < this.conversions.length; i++) {
            int methodPriorityDifference = SignatureUtilities.getPriority(this.conversions[i]) - SignatureUtilities.getPriority(o.conversions[i]);
            if (methodPriorityDifference != 0) {
                return methodPriorityDifference;
            }
        }

        return 0;
    }
}