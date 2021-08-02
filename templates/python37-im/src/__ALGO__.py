import Algorithmia
# from PIL import Image
import shutil
from subprocess import Popen, PIPE
from uuid import uuid4
import os

ALGO_IMAGE_DIRECTORY = 'data://zeryx/collection/'

client = Algorithmia.client()


def convert_image(remote_file, client, output_extension):
    unique_prefix = str(uuid4())  # unique name to assign to temporary local files
    local_images = []  # list to hold the resulting names of images
    resolution = 200
    with client.file(remote_file).getFile() as f:
        local_file = f.name
    output_prefix = "/tmp/" + unique_prefix
    proc = Popen(f"convert --density {resolution} {local_file} {output_prefix}-%03d.{output_extension}".split(' '), stdout=PIPE, stderr=PIPE)
    output, err = proc.communicate()
    for line in err.decode('utf-8').splitlines():
        if "Exception" in line:
            raise Exception(str(line))
    for file in os.listdir("/tmp"):
        if file.startswith(output_prefix):
            local_images.append(str(file))

    # with open(local_file, 'rb') as f:
    #     with Magic(blob=f.read(), resolution=200) as image_stream:
    #         page_count = len(image_stream.sequence)
    #         for page in range(page_count):
    #             with Magic(image_stream.sequence[page]) as image:
    #                 image.make_blob(output_extension)
    #                 image_name = unique_prefix + '_' + str(page + 1) + '.' + output_extension
    #                 image.save(filename='/tmp/' + image_name)
    #                 local_images.append(image_name)
    os.remove(local_file)
    return local_images


def apply(json):
    remote_file = json["inputFile"]  # file stored on Algorithmia's datastore (not local to docker)
    output_extension = json['outputExtension']

    converted_images = convert_image(remote_file, client,
                                     output_extension)  # result will be a list of local image names (could be a list of one)
    results = []
    for i in converted_images:
        full_remote_path = ALGO_IMAGE_DIRECTORY + i
        client.file(full_remote_path).putFile('/tmp/' + i)
        results.append(full_remote_path)
        os.remove('/tmp/' + i)
    delete_files = []

    # Remove imagemagick files on /tmp
    for f in os.listdir('/tmp'):
        if f.lower().startswith('magick'):
            delete_files.append(os.path.join('/tmp', f))
            os.remove(os.path.join('/tmp', f))

    result = {'original_name': i,
              'delete_files': delete_files,
              'space': shutil.disk_usage('/tmp/'),
              'tmp_contents': os.listdir('/tmp/')
              }
    return result