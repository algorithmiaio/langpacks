package com.algorithmia.algorithmHandler;
import com.google.gson.*;

public class AlgorithmHandler<INPUT, STATE, OUTPUT> {

    @FunctionalInterface
    public interface BifunctionWithException<INPUT, STATE, OUTPUT> {
        OUTPUT apply(INPUT t, STATE j) throws Throwable;
    }

    @FunctionalInterface
    public interface FunctionWithException<INPUT, OUTPUT>{
        OUTPUT apply(INPUT t) throws Throwable;
    }

    @FunctionalInterface
    public interface SupplierWithException<STATE> {
        STATE apply() throws Throwable;
    }


    private BifunctionWithException<INPUT, STATE, OUTPUT> applyWState;
    private FunctionWithException<INPUT, OUTPUT> apply;
    private SupplierWithException<STATE> loadFunc;
    private STATE state;



    private void Execute(STATE state){
        RequestHandler<INPUT> in = new RequestHandler<>();
        ResponseHandler<OUTPUT> out = new ResponseHandler<>();
        try {
            Request<INPUT> req = in.GetNextRequest();
            while (req != null) {
                OUTPUT output = this.applyWState.apply(req.data, state);
                out.writeToPipe(output);
                req = in.GetNextRequest();
            }
        } catch (Throwable e){
            out.writeErrorToPipe(e);
        }
    }


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
    public void run(){
        if (this.applyWState != null && this.loadFunc != null){

        }
        System.out.println(this.applyWState.getClass());
        System.out.println(this.loadFunc.getClass());
        System.out.println("Not implemented");
    }



}
