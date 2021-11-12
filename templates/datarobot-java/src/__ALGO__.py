import csv
import Algorithmia
from Algorithmia import ADK
from py4j.java_gateway import JavaGateway, launch_gateway

def load(model_data):
    launch_gateway(port=25333)
    local_jar = model_data.get_model("sentiment_model")
    gateway = JavaGateway()
    url_class = gateway.jvm.java.net.URL
    urls = gateway.new_array(url_class, 1)
    urls[0] = gateway.jvm.java.io.File(local_jar).toURI().toURL()
    cl = gateway.jvm.java.net.URLClassLoader(urls)
    predictors_class = gateway.jvm.java.lang.Class.forName("com.datarobot.prediction.Predictors", True, cl)
    args = gateway.new_array(gateway.jvm.java.lang.Class, 1)
    class_name = gateway.jvm.java.lang.Class.forName("java.lang.ClassLoader")
    args[0] = class_name
    params = gateway.new_array(gateway.jvm.java.lang.Object, 1)
    params[0] = cl
    model_invokation = predictors_class.getMethod("getPredictor", args)
    java_model = model_invokation.invoke(None, params)
    return java_model, model_data.client, gateway



def apply(input, state):
    java_model, client, gateway = state
    row_map = gateway.jvm.java.util.HashMap()
    labels = list(java_model.getClassLabels())
    with client.file(input).getFile() as f:
        parser = csv.reader(f)
        headers = next(parser)
        for h, v in zip(headers, parser):
            row_map.put(h, v)
    results = java_model.score(row_map)
    output = {}
    for result in results.keys():
        output[result] = results[result]
    return output


algo = ADK(apply, load)
algo.init()