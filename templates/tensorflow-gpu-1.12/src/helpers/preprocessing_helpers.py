from PIL import Image
import numpy as np

def convert_to_grayscale(image_file, size=None, normalize_to=None, one_hot=False):
    """
    Helper function to load an image in gray scale, resize and rescale it optionally
    Parameters:
    image_file: Path to the image file that needs to be processed.
    size ((default=None): Option tuple (row, col) defining the reshaped image size
    normalize_to (default=None): Optional parameter that will be used to rescale the pixel values to fall within [0, normalize_to]
    one_hot (default=False: Converts the array into 1-dimension
    Returns:
    Numpy array of the specified size represnting the resized and normalized image.
    """
    image = Image.open(image_file).convert('L')
    if size:
        image = image.resize(size)
    image_data = np.asarray(image)
    if normalize_to:
        image_data = (image_data/255.0) * normalize_to
    if one_hot:
        image_data = np.reshape(image_data, image_data.size).astype(np.float32)
    return image_data
