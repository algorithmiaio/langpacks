import yaml
import os
import subprocess

agents_dir = "/opt/mlops-agent/datarobot_mlops_package-8.1.2"

DATAROBOT_API_TOKEN = os.getenv('DATAROBOT_API_TOKEN', None)
DATAROBOT_ENDPOINT = "https://app.datarobot.com"

with open(f'{agents_dir}/conf/mlops.agent.conf.yaml') as f:
    documents = yaml.load(f, Loader=yaml.FullLoader)
    documents['mlopsUrl'] = DATAROBOT_ENDPOINT
    documents['apiToken'] = DATAROBOT_API_TOKEN
with open(f'{agents_dir}/conf/mlops.agent.conf.yaml', 'w') as f:
    yaml.dump(documents)

subprocess.call(f'{agents_dir}/bin/start-agent.sh')
check = subprocess.Popen([f'{agents_dir}/bin/status-agent.sh'], stdout=subprocess.PIPE)
print(check.stdout.readlines())
check.terminate()