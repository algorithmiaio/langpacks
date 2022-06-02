"""Algorithm Development Kit (ADK) algorithm-creation template.

* This template uses Algorithmia's ADK module to provide structure for algorithm development. For details see:
      https://algorithmia.com/developers/algorithm-development/languages/python#what-is-an-algorithm-development-kit-adk
      https://github.com/algorithmiaio/algorithmia-adk-python

* API calls begin at the `apply` function, with the JSON request body deserialized and passed as `input`.

* The instantiation of an `ADK` object is what turns your library code into an algorithm that can run on Algorithmia.

* The `ADK.init` method is what actually starts the algorithm. To explore further, see the source code linked above.

* If your algorithm relies on in-memory data (e.g., large model files) that are the same every time the algorithm
  is executed, you can load those data into a `globals` object in a `load` function and pass that to `apply`, i.e.:

      ...
      def apply(input, globals):
          return "hello {} {}".format(str(input), str(globals["payload"]))

      def load():
          globals = {}
          globals["payload"] = "Loading has been completed."
          return globals

      algorithm = ADK(apply, load)
      ...
"""
from Algorithmia import ADK
from datarobot.mlops.mlops import MLOps
import os
import pandas as pd

os.environ["MLOPS_DEPLOYMENT_ID"] = "6126e05f97b19e6599ac1f39"
os.environ["MLOPS_MODEL_ID"] = "6126df6a78aa5cb610acea36"
os.environ["MLOPS_SPOOLER_TYPE"] = "FILESYSTEM"
os.mkdir("/tmp/ta")
os.environ["MLOPS_FILESYSTEM_DIRECTORY"] = "/tmp/ta"
mlops = MLOps().init()


def apply(input):
    df = pd.DataFrame(columns=['id', 'values'])
    df.loc[0] = ["abcd", 0.25]
    association_ids = df.iloc[:, 0].tolist()
    reporting_predictions = [0.25]
    mlops.report_deployment_stats(100, 15)
    mlops.report_predictions_data(frame_df=df, predictions=reporting_predictions, association_ids=association_ids)
    return reporting_predictions


algorithm = ADK(apply)
algorithm.init("Algorithmia")
