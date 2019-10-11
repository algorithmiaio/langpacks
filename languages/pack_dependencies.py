import os
import sys
import json
from shutil import copy
from uuid import uuid4


def load_filenames_file(path):
    with open(path) as f:
        files = set(f.readlines())
    return files


def find_differences(before, after):
    difference = after - before
    return difference


def create_export_depot(diff_files, workspace_path, manifest_path):
    os.mkdir(workspace_path)
    manifest = []
    for original_file_path in diff_files:
        original_file_path = original_file_path.replace('\n', '')
        print(original_file_path)
        temp_name = str(uuid4())
        temp_path = "{}/{}".format(workspace_path, temp_name)
        print(temp_path)
        item = {'source': temp_path, 'destination': original_file_path}
        manifest.append(item)
        copy(original_file_path, temp_path)
    with open(manifest_path, 'w') as f:
        json.dump(manifest, f)


if __name__ == "__main__":
    before_data_file_path = sys.argv[1]
    after_data_file_path = sys.argv[2]
    depot_path = sys.argv[3]
    manifest_path = sys.argv[4]
    before_files = load_filenames_file(before_data_file_path)
    after_files = load_filenames_file(after_data_file_path)
    diff_files = find_differences(before_files, after_files)
    create_export_depot(diff_files, depot_path, manifest_path)
