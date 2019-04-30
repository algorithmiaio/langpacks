package com.algorithmia.algorithmHandler;
import com.google.gson.*;

import java.io.Console;

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


    private void Load() throws Throwable{
        state = this.loadFunc.apply();
        System.out.println("PIPE_INIT_COMPLETE");
        System.out.flush();
    }

    private void ExecuteWithoutState(RequestHandler<INPUT> in, ResponseHandler out) throws Throwable {
        Request<INPUT> req = in.GetNextRequest();
        while(req != null){
            OUTPUT output = this.apply.apply(req.data);
            out.writeToPipe(output);
            req = in.GetNextRequest();
        }
    }

    private void ExecuteWithState(RequestHandler<INPUT> in, ResponseHandler out) throws Throwable{
        Request<INPUT> req = in.GetNextRequest();
        while (req != null) {
            OUTPUT output = this.applyWState.apply(req.data, state);
            out.writeToPipe(output);
            req = in.GetNextRequest();
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
    public void run() throws java.io.FileNotFoundException{
        RequestHandler<INPUT> in = new RequestHandler<>();
        ResponseHandler out = new ResponseHandler();
        try {

            if(this.applyWState != null && this.loadFunc != null) {
                Load();
                ExecuteWithState(in, out);
            } else {
                ExecuteWithoutState(in, out);
            }

    } catch (Throwable e){
        out.writeErrorToPipe(e);
    }
    }



}
