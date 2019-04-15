package com.algorithmia.example;

import java.util.HashMap;
import java.util.function.Function;
import java.util.function.Supplier;

@FunctionalInterface
interface FunctionWithException<T, R> {
    R apply(T t) throws Exception;
}

@FunctionalInterface
interface SupplierWithException<R> {
    R apply() throws Exception;
}


public class AlgorithmPipe<I, O> {

    private Function<I, O>  applyFunc;
    private Supplier<HashMap<String, Object>> loadFunc;
    private HashMap<String, Object> context;

    private <I, O> Function<I, O> applyHandler(FunctionWithException<I, O> fe) {
        return arg -> {
            try {
                return fe.apply(arg);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        };
    }

    private <J> Supplier<J> loadHandler(SupplierWithException<J> fe) {
        return () -> {
            try {
                return fe.apply();
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        };
    }
    public AlgorithmPipe(){ }

    public AlgorithmPipe(FunctionWithException<I, O> applyFunc){
        this.applyFunc = applyHandler(applyFunc);
    }

    public void setApply(FunctionWithException<I, O> func){
        applyFunc = applyHandler(func);
    }

    public void setLoad(SupplierWithException<HashMap<String, Object>> func){

        loadFunc = loadHandler(func);
    }
    public void run(){
        System.out.println("Not implemented");
    }

}
