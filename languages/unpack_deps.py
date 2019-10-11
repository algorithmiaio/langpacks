import os
import sys
import json
from shutil import copyfile


def load_manifest(file_path):
    with open(file_path) as f:
        manifest = json.load(f)
    return manifest

def copy_from_manifest(manifest_data):
    source = manifest_data['source']
    destination = manifest_data['destination']
    dest_dir = '/'.join(destination.split('/')[:-1])
    print(dest_dir)
    print(destination)
    os.makedirs(dest_dir, exist_ok=True)
    try:
        copyfile(source, destination)
    except Exception as e:
        print(e)
        pass




if __name__ == '__main__':
    manifest_path = sys.argv[1]
    manifest = load_manifest(manifest_path)
    for data in manifest:
        copy_from_manifest(data)
