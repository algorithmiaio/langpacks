package com.algorithmia.example;

import java.io.Console;
import java.util.HashMap;
import java.util.function.BiFunction;
import java.util.function.Function;
import java.util.function.Supplier;

@FunctionalInterface
interface FunctionWithException<T1, T2, R> {
    R apply(T1 t, T2 j) throws Exception;
}

@FunctionalInterface
interface SupplierWithException<R> {
    R apply() throws Exception;
}


public class AlgorithmHandler<I1, O, I2> {

    private BiFunction<I1, I2, O> applyFunc;
    private Supplier<I2> loadFunc;
    private HashMap<String, Object> context;

    private <T, J, L> BiFunction<T, J, L> applyHandler(FunctionWithException<T, J, L> fe) {
        return (arg1, arg2) -> {
            try {
                return fe.apply(arg1, arg2);
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
    public AlgorithmHandler(){ }

    public AlgorithmHandler(FunctionWithException<I1,I2, O> applyFunc, SupplierWithException<I2> loadFunc){
        this.applyFunc = applyHandler(applyFunc);
        this.loadFunc = loadHandler(loadFunc);
    }

    public AlgorithmHandler(FunctionWithException<I1, I2, O> applyFunc){
        this.applyFunc = applyHandler(applyFunc);
    }

    public void setApply(FunctionWithException<I1, I2, O> func){
        applyFunc = applyHandler(func);
    }

    public void setLoad(SupplierWithException<I2> func){

        loadFunc = loadHandler(func);
    }
    public void run(){
        System.out.println(this.applyFunc.getClass());
        System.out.println(this.loadFunc.getClass());
        System.out.println("Not implemented");
    }

}
