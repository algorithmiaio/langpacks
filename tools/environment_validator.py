#!/usr/bin/env python3
from os import path
import os
import argparse
import shutil
from uuid import uuid4
import json
import pycurl
from io import StringIO, BytesIO

import docker
from docker.models.images import Image
from docker.models.containers import _create_container_args

from template_manager import generate_intermediate_image, generate_compile_image

DIR_PATH_TO_PACKAGES = "libraries"
DIR_PATH_TO_DEP_TEMPLATES = "templates"
DIR_PATH_TO_LANGUAGES = "languages"
WORKSPACE_PATH = "/tmp/validator_cache"
LOCAL_PORT = 9999

"""
This script creates your packageset docker images, and runs them the same way that langserver does. This allows us to fully replicate
the runtime environment, and verify that your new package (whether it's a dependency based package, or language based) will function properly .

If you haven't deployed your modifications to the algorithmia-client, plese do so before. Otherwise you'll have to
pipe in your modified client into the docker containers.
"""

def build_image(docker_client, dockerfile_name, workspace_path, image_tag):
    """
    This function builds a docker image based on a defined dockerfile, it always removes intermediate containers to reduce system bloat.
    Docker requires a workspace, where everything is located within. See "prepare_workspace" for more info.
    :param docker_client: The docker python client
    :param dockerfile_name: Name of the dockerfile in the root of the workspace
    :param workspace_path: Path to the workspace (default is /tmp/validator_cache)
    :param image_tag: The desired image tag name (useful for demolition later)
    :return: A docker image object (see docker sdk for more info)
    """
    try:
        image, _ = docker_client.images.build(dockerfile=dockerfile_name, path=workspace_path, tag=image_tag, rm=True)
        return image
    except docker.errors.BuildError as e:
        for line in e.build_log:
            if 'stream' in line:
                print(line)
        raise e


def create_intermediate_image(docker_client, base_image, dependencies, workspace_path, mode):
    """
    Creates either a buildtime or runtime image, based on mode. Uses function in 'template_manager' to auto-generate the
    right dockerfile based on the provided context. For more info on the template files, check out
    `Dockerfile.builder.j2` and `Dockerfile.runner.j2` in the languages directory.
    :param docker_client: The docker python client
    :param base_image: The base image type in which to stage your docker container from, defaults to "ubuntu:16.04", but can be any standard base image.
    :param dependencies: A list of dependencies that this intermediate image depends on, excluding language components. (eg: pytorch-1.0.0, spacy-2.0.18, etc)
    :param workspace_path: Path to the workspace (default is /tmp/validator_cache)
    :param mode: What type of intermediate image this is, either "runtime" or "buildtime".
    :return: A docker image object (see docker sdk for more info)
    """
    tag = "validator-{}-{}".format(str(mode), str(uuid4()))
    image_name = "{}.Dockerfile".format(tag)
    full_image_path = "{}/{}".format(workspace_path, image_name)
    generate_intermediate_image(base_image, dependencies, full_image_path, mode)
    print("building {} image".format(mode))
    return build_image(docker_client, image_name, workspace_path, tag)


def create_final_image(client, builder_image, runner_image, workspace_path,
                       config, local_testing_destination=None):
    """
    Creates a final image, which uses multi-stage compilation (https://docs.docker.com/develop/develop-images/multistage-build/) which are pretty cool.
    Very similar to the 'create_intermediate_image' but uses the build products as stages for the final product. For more info on the template file,
    check out "Dockerfile.compile.j2' in the languages directory.
    :param client: The docker python client
    :param builder_image: The buildtime docker image object generated from 'create_intermediate_image'
    :param runner_image: The runtime docker image object generated from 'create_intermediate_image'
    :param workspace_path: Path to the workspace (default is /tmp/validator_cache)
    :param config: The desired languages config data stored in the config.json file (dictionary)
    :param local_testing_destination: If you provide local system dependencies, this defines where that should be placed in the final docker image, before compilation.
    :return: A docker image object (see docker sdk for more info)
    """
    tag = "validator-{}-{}".format("final", str(uuid4()))
    image_name = "{}.Dockerfile".format(tag)
    full_image_path = "{}/{}".format(workspace_path, image_name)
    if local_testing_destination:
        config['local_dependency_dest_path'] = local_testing_destination
        config['local_dependency_src_path'] = "dependency"
    generate_compile_image(builder_image, runner_image, config, full_image_path)
    print("building final image")
    return build_image(client, image_name, workspace_path, tag)


def run_algorithm_container(client, image, nvidia_support, algorithmia_api_key):

    raw_args = {}

    if nvidia_support:
        device_request = {
            'Driver': 'nvidia',
            'Capabilities': [['gpu'], ['nvidia'], ['compute'], ['compat32'], ['graphics'], ['utility'], ['video'],
                             ['display']],  # not sure which capabilities are really needed
            'Count': -1,  # enable all gpus
            'NVIDIA_VISIBLE_DEVICES': '-1',
        }
    else:
        device_request = None

    if isinstance(image, Image):
        image = image.id
    raw_args['image'] = image
    # kwargs['command'] = command
    raw_args['version'] = client.containers.client.api._version
    raw_args['ports'] = {9999: 9999}
    container_args = _create_container_args(raw_args)
    # modification to the original create function
    container_args['host_config'] = client.api.create_host_config(port_bindings={LOCAL_PORT: ("127.0.0.1", LOCAL_PORT)})

    if device_request is not None:
        container_args['host_config']['DeviceRequests'] = [device_request]
    # end modification
    container_args['detach'] = True
    if algorithmia_api_key:
        container_args['environment'] = {}
        container_args['environment']['ALGORITHMIA_API_KEY'] = algorithmia_api_key

    resp = client.api.create_container(**container_args)
    client.api.start(resp['Id'])
    return resp['Id']

def prepare_workspace(workspace_path, template_path, local_cached_dependency_source_path=None):
    """
    Creates and prepares a workspace for docker, docker requires all used files by the docker build operation to be relative to this workspace directory.
    If you desire a file to be copied into a docker image, but it's not in this directory - a file not found error will be thrown.
    
    Workspace is terminated upon termination of this script
    :param workspace_path: System path, default is "/tmp/validator_cache"
    :param template_path: Relative path to your final image template, eg: languages/java11/template
    :param local_cached_dependency_source_path: If you're using local dependencies for testing purposes, this is absolute the source path on your system, eg: /home/zeryx/.m2
    :return: None
    """
    algosource_path = path.join(workspace_path, "algosource")
    shutil.copytree(path.join(os.getcwd(), "libraries"), workspace_path)
    shutil.copytree(template_path, algosource_path)
    if local_cached_dependency_source_path:
        shutil.copytree(local_cached_dependency_source_path, path.join(workspace_path, "dependency"))


def stop_and_kill_containers(docker_client, all=False):
    """
    Kills all docker containers, if all is =true, it kills all containers whether running or not
    :param docker_client: The docker python client
    :param all: Boolean variable defining whether we destroy 'all' docker containers, or just running ones
    :return: None
    """
    containers = docker_client.containers.list(all=all, ignore_removed=True)
    for container in containers:
        try:
            container.remove(force=True)
        except docker.errors.APIError:
            pass


def kill_dangling_images(docker_client):
    """
    Kills all dangling images, to free up disk space
    :param docker_client: The docker python client
    :return: None
    """
    images = docker_client.images.list()
    for image in images:
        if len(image.tags) == 0:
            docker_client.images.remove(image.id, force=True)


def run_tests(client, container, input_lines, expected_lines):
    test_status = True
    logs = client.api.attach(container, stream=True, logs=True, stdout=True, stderr=True)
    for log in logs:
        if b"Listening on port 9999" in log:
            break
    for input, expected in zip(input_lines, expected_lines):
        buffer = BytesIO()
        if input == " " or input == "" or expected == " " or expected == "":
            break
        c = pycurl.Curl()
        c.setopt(pycurl.URL, "localhost:9999")
        c.setopt(pycurl.HTTPHEADER, ['Content-Type: application/json'])
        c.setopt(pycurl.POST, 1)
        c.setopt(pycurl.TIMEOUT_MS, 3000)
        c.setopt(pycurl.READDATA, StringIO(input))
        c.setopt(pycurl.POSTFIELDSIZE, len(input))
        c.setopt(pycurl.WRITEDATA, buffer)
        c.perform()
        c.close()
        output = buffer.getvalue()
        output = json.loads(output)
        expected = json.loads(expected)
        if output['result'] == expected['result']:
            print("pass")
        else:
            print("fail")
            print("output: {}\nexpected: {}".format(output, expected))
            test_status = False
    if test_status:
        print("All tests successful")
    else:
        print("At least one test failed")

def load_lines(testing_file_path):
    with open(testing_file_path) as f:
        lines = f.read()
    lines = lines.split("\n")
    return lines


def main(base_image, language_general_name, language_specific_name,
         template_type, template_name, dependencies, local_src, local_dest, cleanup_after, nvidia_support,
         algorithmia_api_key, automatic_testing):

    client = docker.from_env()

    if template_type == "dependency":
        template_path = path.join(os.getcwd(), DIR_PATH_TO_DEP_TEMPLATES, template_name)
    elif template_type == "language":
        template_path = path.join(os.getcwd(), DIR_PATH_TO_LANGUAGES, template_name, "template")
    else:
        raise Exception("template type must be either 'dependency' or 'language")
    prepare_workspace(WORKSPACE_PATH, template_path, local_src)

    try:
        if dependencies:
            runtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "runtime")] + dependencies
            buildtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "buildtime")] + dependencies
        else:
            runtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "runtime")]
            buildtime_dirs = [language_specific_name, "{}-{}".format(language_general_name, "buildtime")]

        runtime_image = create_intermediate_image(client, base_image, runtime_dirs, WORKSPACE_PATH, "runtime")
        buildtime_image = create_intermediate_image(client, base_image, buildtime_dirs, WORKSPACE_PATH, "buildtime")

        with open(path.join(os.getcwd(), DIR_PATH_TO_LANGUAGES, language_general_name, "config.json")) as f:
            config = json.load(f)

        compile_image = create_final_image(client, buildtime_image.id, runtime_image.id, WORKSPACE_PATH, config, local_dest)
        container = run_algorithm_container(client, compile_image, nvidia_support, algorithmia_api_key)

        if automatic_testing:
            input_file = path.join(template_path,"docker_test_input")
            eval_file = path.join(template_path, "docker_test_output")
            test_inputs = load_lines(input_file)
            test_outputs = load_lines(eval_file)
            run_tests(client, container, test_inputs, test_outputs)
        else:
            logs = client.api.attach(container, stream=True, logs=True, stdout=True, stderr=True)
            print("container started, listening for requests on")
            print("for an example, try passing the following curl command in a separate terminal:\n")
            print("curl localhost:9999 -H 'Content-Type: application/json' -d '{\name\": \"Anthony\"}'")
            for log in logs:
                print(log)
    except Exception as e:
        shutil.rmtree(WORKSPACE_PATH)
        if cleanup_after:
            stop_and_kill_containers(client, True)
            kill_dangling_images(client)
        else:
            stop_and_kill_containers(client)
        raise e
    except KeyboardInterrupt:
        shutil.rmtree(WORKSPACE_PATH)
        if cleanup_after:
            print("cleaning up")
            stop_and_kill_containers(client)
            kill_dangling_images(client)
            print("done")
        else:
            stop_and_kill_containers(client)
        return
    shutil.rmtree(WORKSPACE_PATH)
    if cleanup_after:
        print("cleaning up")
        stop_and_kill_containers(client)
        kill_dangling_images(client)
        print("done")
    else:
        stop_and_kill_containers(client)


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
                                                                                           "For example: pytorch-1.0.0, or java11.")
    parser.add_argument('-d', '--dependency', action="append", dest="dependencies", help="A list builder of all non-language dependency packages that your algorithm needs."
                                                                                         "Language core, buildtime & runtime are included automatically.")
    parser.add_argument('-c', '--clean-up', dest='cleanup', type=bool, help="A boolean variable that if set, forces us to clean up docker containers and images created by this process.")
    parser.add_argument('--local-dependency-src', dest='local_src', help="If using a local cached dependency for testing, is the path towards that dependency on your file system.")
    parser.add_argument('--local-dependency-dest', dest='local_dest', help="If using a local cached dependency for testing, is the path where the dependency will live in the compileLocal image.")
    parser.add_argument('--nvidia-support', type=bool, help="A boolean variable that if set, ensures that the final docker image is started with a nvidia-docker context. Requires 'nvidia-docker' to be installed.")
    parser.add_argument('-k', '--algorithmia-api-key', type=str, help="A string value that when defined, provides the ALGORITHMIA_API_KEY environment variable to the algorithm runtimme, to enable algorithm client access.")
    parser.add_argument('-a', '--automatic-testing', type=bool, help="A boolean variable that when set, runs any automatic test scripts found in the algorithm template, and provides the results to the console.")

    args = parser.parse_args()

    if not args.language_general_name:
        args.language_general_name = args.language_specific_name
    if not args.cleanup:
        args.cleanup = False
    if not (args.local_src and args.local_dest) and (args.local_src or args.local_dest):
        raise Exception("if you're using local dependencies, src & dest must be defined.")
    main(
        base_image=args.base_image,
        language_general_name=args.language_general_name,
        language_specific_name=args.language_specific_name,
        template_type=args.template_type,
        template_name=args.template_name,
        dependencies=args.dependencies,
        local_src = args.local_src,
        local_dest = args.local_dest,
        cleanup_after=args.cleanup,
        nvidia_support=args.nvidia_support,
        algorithmia_api_key=args.algorithmia_api_key,
        automatic_testing=args.automatic_testing,
    )
