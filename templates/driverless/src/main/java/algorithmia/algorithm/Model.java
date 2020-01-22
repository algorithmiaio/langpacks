package algorithmia.algorithm;

import ai.h2o.mojos.runtime.MojoPipeline;
import ai.h2o.mojos.runtime.frame.MojoFrame;
import ai.h2o.mojos.runtime.frame.MojoFrameBuilder;
import ai.h2o.mojos.runtime.frame.MojoRowBuilder;
import com.google.gson.Gson;
import com.google.gson.JsonObject;

import java.net.URL;
import java.net.URLClassLoader;
import java.util.Map;

public class Model {
    private MojoPipeline model = null;
    private Gson gson = new Gson();

    public Model(String mojoPath) throws Exception {
            System.out.println("loading model");
        ClassLoader cl = ClassLoader.getSystemClassLoader();
        System.out.println("loaded classloader");
        URL[] urls = ((URLClassLoader)cl).getURLs();
        System.out.println("got urls");
            for(URL url: urls){
                System.out.println("classname: "+url.getFile());
            }
            model = MojoPipeline.loadFrom(mojoPath);
            System.out.println("loaded model");
    }

    public JsonObject predict(Map input) {
        JsonObject output = new JsonObject();
        if(model != null){
            System.out.println("predicting...");
            MojoFrameBuilder frameBuilder = model.getInputFrameBuilder();
            MojoRowBuilder rowBuilder = frameBuilder.getMojoRowBuilder();
            input.forEach((id, value) -> {
                rowBuilder.setValue((String) id, (String)value);
            });
            frameBuilder.addRow(rowBuilder);

            MojoFrame iframe = frameBuilder.toMojoFrame();

            MojoFrame oframe = model.transform(iframe);
            for(int i=0; i < oframe.getNcols(); i++) {
                String columnName = oframe.getColumnName(i).replaceAll("\\.| ", "_");
                String columnData = gson.toJson(oframe.getColumnData(i)).replaceAll("\\[|\\]", "");
                output.addProperty(columnName, columnData);
            }
        }
        return output;
    }
}
