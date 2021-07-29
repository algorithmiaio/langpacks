import Algorithmia
# from PIL import Image
import shutil
from wand.image import Image as Magic
from uuid import uuid4
import os

ALGO_IMAGE_DIRECTORY = 'data://pl_data_science_build/convert_image_file_type_store/'

client = Algorithmia.client()


def convert_image(local_image_file, output_extension):
    unique_prefix = str(uuid4())  # unique name to assign to temporary local files
    local_images = []  # list to hold the resulting names of images
    with open(local_image_file, 'rb') as f:
        binary_data = f.read()
        with Magic(blob=binary_data, resolution=200) as image_stream:
            page_count = len(image_stream.sequence)
            for page in range(page_count):
                with Magic(image_stream.sequence[page]) as image:
                    image.make_blob(output_extension)
                    image_name = unique_prefix + '_' + str(page + 1) + '.' + output_extension
                    image.save(filename='/tmp/' + image_name)
                    local_images.append(image_name)
    return local_images


def apply(json):
    remote_file = json["inputFile"]  # file stored on Algorithmia's datastore (not local to docker)
    output_extension = json['outputExtension']

    local_input_file = client.file(remote_file).getFile().name  # this is actually something like "/tmp/tmpk89y2ip6"

    converted_images = convert_image(local_input_file,
                                     output_extension)  # result will be a list of local image names (could be a list of one)
    results = []
    for i in converted_images:
        full_remote_path = ALGO_IMAGE_DIRECTORY + i
        client.file(full_remote_path).putFile('/tmp/' + i)
        results.append(full_remote_path)
        os.remove(local_input_file)
        os.remove('/tmp/' + i)
    delete_files = []

    # Remove imagemagick files on /tmp
    for f in os.listdir('/tmp'):
        if f.lower().startswith('magick'):
            delete_files.append(os.path.join('/tmp', f))
            os.remove(os.path.join('/tmp', f))

    result = {'original_name': i,
              'delete_files': delete_files,
              'temp_name': local_input_file,
              'space': shutil.disk_usage('/tmp/'),
              'tmp_contents': os.listdir('/tmp/')
              }
    return result