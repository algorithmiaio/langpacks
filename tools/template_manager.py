from jinja2 import Template
from os import path
from os.path import isfile

DIR_PATH_TO_TEMPATES = "languages"
DIR_PATH_TO_PACKAGES = "libraries"
RUNNER_NAME = "Dockerfile.runner.j2"
BUILDER_NAME = "Dockerfile.builder.j2"
COMPILE_NAME = "Dockerfile.compile.j2"
LANGSERVER_VERSION = "b35efaa7d69d24efcd83e866b7445446f92eb0d6"  # Update this when we change how langserver works.
LANGSERVER_IMAGE = "algorithmiahq/langserver:{}".format(LANGSERVER_VERSION)
RUNNER_PATH = path.join(DIR_PATH_TO_TEMPATES, RUNNER_NAME)
BUILDER_PATH = path.join(DIR_PATH_TO_TEMPATES, BUILDER_NAME)
COMPILE_PATH = path.join(DIR_PATH_TO_TEMPATES, COMPILE_NAME)


class Package:
    def __init__(self, package_name, install_script, dockerfile_path):
        self.package_name = package_name
        self.script = install_script
        if dockerfile_path:
            self.dockerfile = get_dockerfile_as_string(dockerfile_path)
        else:
            raise Exception(
                "dockerfile path not available for package {}".format(package_name)
            )


def get_dockerfile_as_string(file_path):
    with open(file_path, "r") as fileobject:
        stringified = fileobject.read()
    output = stringified.split("\n")
    return output


def get_template(template_path):
    with open(template_path, "r") as fileobject:
        template_string = fileobject.read()
    template = Template(template_string)
    return template


def save_generated_template(template, output_path):
    with open(output_path, "w") as fileobject:
        fileobject.write(template)
    return output_path


def check_if_exists(filepath):
    if isfile(filepath):
        return filepath
    else:
        return None


def generate_compile_image(
    builder_image_name, runner_image_name, config_data, output_file_path
):
    raw_template = get_template(COMPILE_PATH)
    generated_template = raw_template.render(
        builder_image=builder_image_name,
        runner_image=runner_image_name,
        config=config_data,
        local=True,
    )
    save_generated_template(generated_template, output_file_path)
    print(
        "completed template construction, file available at {}".format(output_file_path)
    )
    return output_file_path


def generate_intermediate_image(base_image, package_dirs, output_file_path, mode):
    if mode == "runtime":
        raw_template = get_template(RUNNER_PATH)
    elif mode == "buildtime":
        raw_template = get_template(BUILDER_PATH)
    else:
        raise Exception(
            "we did not recieve a valid 'mode', it must be either 'runtime' or 'buildtime'."
        )
    packages = []
    for dir in package_dirs:
        dockerfile_path = path.join(DIR_PATH_TO_PACKAGES, dir, "Dockerfile")
        installer_path = path.join(DIR_PATH_TO_PACKAGES, dir, "install.sh")
        dockerfile_path = check_if_exists(dockerfile_path)
        installer_path = check_if_exists(installer_path)

        package = Package(dir, installer_path, dockerfile_path)
        packages.append(package)
    generated_template = raw_template.render(
        packages=packages,
        base_image=base_image,
        langpacks_version="",
        langserver_image=LANGSERVER_IMAGE,
    )
    save_generated_template(generated_template, output_file_path)
    print(
        "completed template construction, file available at {}".format(output_file_path)
    )
    return output_file_path
