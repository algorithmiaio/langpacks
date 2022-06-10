from Algorithmia import ADK
from datarobot.mlops.mlops import MLOps
import os
import pandas as pd


def apply(input):
    df = pd.DataFrame(columns=['id', 'values'])
    df.loc[0] = ["abcd", 0.25]
    association_ids = df.iloc[:, 0].tolist()
    reporting_predictions = [0.25]
    mlops.report_deployment_stats(100, 15)
    mlops.report_predictions_data(features_df=df, predictions=reporting_predictions, association_ids=association_ids)
    return reporting_predictions


algorithm = ADK(apply, mlops=True)
algorithm.init("Algorithmia")
