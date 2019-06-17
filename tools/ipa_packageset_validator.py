#!/usr/bin/env python3
from os import path
import argparse
from uuid import uuid4
import docker
from docker.types import Mount
from tools.ipa_packageset_builder import build

DIR_PATH_TO_PACKAGES = "libraries"
DIR_PATH_TO_DEP_TEMPLATES = "templates"
DIR_PATH_TO_LANGUAGES = "languages"
LOCAL_PORT = 9999


# This script creates your packageset docker images, and runs them the same way that langserver does. This allows us to fully replicate
# the runtime environment, and verify that your new package (whether it's a dependency based package, or language based) will function properly .

# If you haven't deployed your modifications to the algorithmia-client, plese do so before. Otherwise you'll have to
# pipe in your modified client into the docker containers.


def get_language_dirs(langauge_name, mode):
    expected_lang_names = [langauge_name, "{}-{}".format(langauge_name, mode)]
    return expected_lang_names


def create_image(client, base_image, dependencies, mode):
    image_name = str(uuid4())
    image_path = build(base_image, dependencies, "/tmp/{}.Dockerfile".format(image_name), mode)
    image, error = client.images.build(path=image_path, tag=image_name)
    if error:
        raise Exception("we ran into an error during building, {}".format(error))
    else:
        return image


def run_builder(client, builder_image, template_path):
    mount = Mount(target="/opt/algorithm", source=template_path, type="bind", read_only=False)
    container, error = client.containers.create(image=builder_image, mounts=[mount])
    if error:
        raise Exception("failed to create container, {}".format(error))
    else:
        container.start()
        code, output = container.exec_run(cmd="sh /usr/local/bin/algorithm-build")
        return output


def main(base_image, language_name, dependencies, template_type, template_name):
    client = docker.from_env()

    # create dockerfiles

    runtime_dirs = dependencies + get_language_dirs(language_name, "runtime")
    buildtime_dirs = dependencies + get_language_dirs(language_name, "buildtime")

    runtime_path = create_image(client, base_image, runtime_dirs, "runtime")
    buildtime_path = create_image(client, base_image, buildtime_dirs, "buildtime")

    # config = load_lang_config(language_name)

    if template_type == "dependency":
        template_path = path.join("", DIR_PATH_TO_DEP_TEMPLATES, template_name)
    elif template_type == "language":
        template_path = path.join("", DIR_PATH_TO_LANGUAGES, template_name, "template")
    else:
        raise Exception("template type must be either 'dependency' or 'language")
    artifacts = run_builder(client, buildtime_path)



if __name__ == "main__":
    main("ubuntu:16.04", "java11", list(), "language", "java11")


