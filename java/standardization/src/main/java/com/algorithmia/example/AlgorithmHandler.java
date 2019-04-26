package com.algorithmia.example;


public class AlgorithmHandler<INPUT, STATE, OUTPUT> {

    @FunctionalInterface
    interface BifunctionWithException<INPUT, STATE, OUTPUT> {
        OUTPUT apply(INPUT t, STATE j) throws Exception;
    }

    @FunctionalInterface
    interface FunctionWithException<INPUT, OUTPUT>{
        OUTPUT apply(INPUT t) throws Exception;
    }

    @FunctionalInterface
    interface SupplierWithException<STATE> {
        STATE apply() throws Exception;
    }


    private BifunctionWithException<INPUT, STATE, OUTPUT> applyWState;
    private FunctionWithException<INPUT, OUTPUT> apply;
    private SupplierWithException<STATE> loadFunc;
    private STATE context;


    public AlgorithmHandler(BifunctionWithException<INPUT, STATE, OUTPUT> applyWState, SupplierWithException<STATE> loadFunc){
        this.applyWState = applyWState;
        this.loadFunc = loadFunc;
    }

    public AlgorithmHandler(BifunctionWithException<INPUT, STATE, OUTPUT> applyWState){
        this.applyWState = applyWState;
    }

    public AlgorithmHandler(FunctionWithException<INPUT, OUTPUT> apply){
        this.apply = apply;
    }

    public void setLoad(SupplierWithException<STATE> func){

        loadFunc = func;
    }
    public void run() throws Exception{
        loadFunc.apply();
        System.out.println(this.applyWState.getClass());
        System.out.println(this.loadFunc.getClass());
        System.out.println("Not implemented");
    }

}
