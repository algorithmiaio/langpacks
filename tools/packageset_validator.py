#!/usr/bin/env python3
from os import path
import os
import argparse
import shutil
from uuid import uuid4
import docker
import json
from template_manager import build, build_compile_image

DIR_PATH_TO_PACKAGES = "libraries"
DIR_PATH_TO_DEP_TEMPLATES = "templates"
DIR_PATH_TO_LANGUAGES = "languages"
WORKSPACE_PATH = "/tmp/testing"
LOCAL_PORT = 9999


# This script creates your packageset docker images, and runs them the same way that langserver does. This allows us to fully replicate
# the runtime environment, and verify that your new package (whether it's a dependency based package, or language based) will function properly .

# If you haven't deployed your modifications to the algorithmia-client, plese do so before. Otherwise you'll have to
# pipe in your modified client into the docker containers.


def create_image(client, base_image, dependencies, workspace_path, mode):
    tag = str(uuid4())
    image_name = "{}.Dockerfile".format(tag)
    build(base_image, dependencies, "{}/{}".format(workspace_path, image_name), mode)
    print("building {} image".format(mode))
    image, _ = client.images.build(dockerfile=image_name, path=workspace_path, tag=tag)
    return image


def create_compile_image(client, builder_image, runner_image, workspace_path, config):
    tag = str(uuid4())
    image_name = "{}.Dockerfile".format(tag)
    build_compile_image(builder_image, runner_image, config, "{}/{}".format(workspace_path, image_name))
    print("building compiletime image (last build stage)")
    image, _ = client.images.build(dockerfile=image_name, path=workspace_path, tag=tag)
    return image


def run_compiler(client, compiler_image):
    # mount = Mount(target="/opt/algorithm", source=template_path, type="bind", read_only=False)
    print("loading compiletime image into container")
    container = client.containers.run(image=compiler_image.id, ports={LOCAL_PORT: ("127.0.0.1", LOCAL_PORT)}, detach=True)
    return container


def prepare_workspace(workspace_path, template_path):
    try:
        algosource_path = path.join(workspace_path, "algosource")
        shutil.copytree(path.join(os.getcwd(), "libraries"), workspace_path)
        shutil.copytree(template_path, algosource_path)
    except FileExistsError:
        pass
    return True


def stop_and_kill_containers(client):
    containers = client.containers.list()
    for container in containers:
        container.kill()
    return True


def main(base_image, language_general_name, language_specific_name,
         template_type, template_name, dependencies):

    client = docker.from_env()

    if template_type == "dependency":
        template_path = path.join(os.getcwd(), DIR_PATH_TO_DEP_TEMPLATES, template_name)
    elif template_type == "language":
        template_path = path.join(os.getcwd(), DIR_PATH_TO_LANGUAGES, template_name, "template")
    else:
        raise Exception("template type must be either 'dependency' or 'language")
    prepare_workspace(WORKSPACE_PATH, template_path)

    try:
        if dependencies:
            runtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "runtime")] + dependencies
            buildtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "buildtime")] + dependencies
        else:
            runtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "runtime")]
            buildtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "buildtime")]

        runtime_image = create_image(client, base_image, runtime_dirs, WORKSPACE_PATH, "runtime")
        buildtime_image = create_image(client, base_image, buildtime_dirs, WORKSPACE_PATH, "buildtime")

        with open(path.join(os.getcwd(), DIR_PATH_TO_LANGUAGES, language_general_name, "config.json")) as f:
            config = json.load(f)

        compile_image = create_compile_image(client, buildtime_image.id, runtime_image.id, WORKSPACE_PATH, config)
        container = run_compiler(client, compile_image)
        logs = container.attach(stream=True, logs=True, stdout=True, stderr=True)
        print("container started, listening for requests on")
        for log in logs:
            print(log)
    except Exception as e:
        shutil.rmtree(WORKSPACE_PATH)
        stop_and_kill_containers(client)
        raise e
    except KeyboardInterrupt:
        print("cleaning up")
        shutil.rmtree(WORKSPACE_PATH)
        stop_and_kill_containers(client)
        print("done")


if __name__ == "__main__":

    parser = argparse.ArgumentParser(description='Creates a simulation of the IPA / langserver / algorithm interface. \n'
                                                 'Use this to test new language, and new dependency packages.')
    parser.add_argument('-b', '--base-image', dest='base_image', type=str,
                        default="ubuntu:16.04", help="the linux base image to build your packageset on top of. Usually an ubuntu version."
                                                     "Defaults to 'ubuntu:16.04'")
    parser.add_argument('-g', '--language-general-name', dest='language_general_name', help="The general name for your language, "
                                                                                            "if multiple minor versions can use the same runtime/buildtime."
                                                                                            "For example: Python3 or Python2."
                                                                                            "Defaults to the value defined for --language-specific-name")
    parser.add_argument('-s', '--language-specific-name', dest='language_specific_name', required=True, help="The fully specified name of your language."
                                                                                                             "For example: Python37. or csharp-dot-core2.")
    parser.add_argument('-t', '--template-type', dest='template_type', required=True, help="The type of template we're using, this can be either:"
                                                                                           "'dependency' - for frameworks/etc "
                                                                                           "'language' - for new language implementations & modifications")
    parser.add_argument('-n', '--template-name', dest='template_name', required=True, help="The name of your template directory."
                                                                                           "For example: pytorch-1.0.0, orjava11.")
    parser.add_argument('-d', '--dependency', action="append", dest="dependencies", help="A list builder of all non-language dependency packages that your algorithm needs."
                                                                                         "Language core, buildtime & runtime are included automatically.")
    args = parser.parse_args()

    if not args.language_general_name:
        args.language_general_name = args.language_specific_name

    main(
        base_image=args.base_image,
        language_general_name=args.language_general_name,
        language_specific_name=args.language_specific_name,
        template_type=args.template_type,
        template_name=args.template_name,
        dependencies=args.dependencies
    )
