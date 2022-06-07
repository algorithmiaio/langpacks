from Algorithmia import ADK
from datarobot.mlops.mlops import MLOps
import os
import pandas as pd
from .configurator import *

os.environ["MLOPS_DEPLOYMENT_ID"] = "6126e05f97b19e6599ac1f39"
os.environ["MLOPS_MODEL_ID"] = "6126df6a78aa5cb610acea36"
os.environ["MLOPS_SPOOLER_TYPE"] = "FILESYSTEM"
os.mkdir("/tmp/ta")
os.environ["MLOPS_FILESYSTEM_DIRECTORY"] = "/tmp/ta"
mlops = MLOps().init()


def apply():
    df = pd.DataFrame(columns=['id', 'values'])
    df.loc[0] = ["abcd", 0.25]
    association_ids = df.iloc[:, 0].tolist()
    reporting_predictions = [0.25]
    mlops.report_deployment_stats(100, 15)
    mlops.report_predictions_data(features_df=df, predictions=reporting_predictions, association_ids=association_ids)
    return reporting_predictions


algorithm = ADK(apply)
algorithm.init("Algorithmia")
