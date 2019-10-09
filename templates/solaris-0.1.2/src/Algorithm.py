import Algorithmia
# API calls will begin at the apply() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

import numpy as np
import solaris as sol
from shapely.wkt import loads

def rescale_auto():
    expected_result = np.array([[[  0,   0,   0],
                                   [ 10,  10,  10],
                                   [ 21,  21,  21],
                                   [ 31,  31,  31],
                                   [ 42,  42,  42]],

                                  [[ 53,  53,  53],
                                   [ 63,  63,  63],
                                   [ 74,  74,  74],
                                   [ 85,  85,  85],
                                   [ 95,  95,  95]],

                                  [[106, 106, 106],
                                   [116, 116, 116],
                                   [127, 127, 127],
                                   [138, 138, 138],
                                   [148, 148, 148]],

                                  [[159, 159, 159],
                                   [170, 170, 170],
                                   [180, 180, 180],
                                   [191, 191, 191],
                                   [201, 201, 201]],

                                  [[212, 212, 212],
                                   [223, 223, 223],
                                   [233, 233, 233],
                                   [244, 244, 244],
                                   [255, 255, 255]]], dtype='uint8')
    im_arr = np.arange(5*5*3, 5*5*6).reshape(5, 5, 3).astype('uint16')
    normalized_arr = sol.utils.io.preprocess_im_arr(im_arr, 'uint16', rescale=True)

    assert np.array_equal(normalized_arr, expected_result)


def reproject_from_wkt():
    input_str = "POLYGON ((736687.5456353347 3722455.06780279, 736686.9301210654 3722464.96326352, 736691.6397869177 3722470.9059681, 736705.5443059544 3722472.614050498, 736706.8992101226 3722462.858909504, 736704.866059878 3722459.457111885, 736713.1443474176 3722452.103498172, 736710.0312805283 3722447.309985571, 736700.3886167214 3722454.263705271, 736698.4577440721 3722451.98534527, 736690.1272768064 3722451.291527834, 736689.4108667439 3722455.113813923, 736687.5456353347 3722455.06780279))"
    result_str = "POLYGON ((-84.4487639 33.6156071, -84.44876790000001 33.6156964, -84.4487156 33.61574889999999, -84.44856540000001 33.6157612, -84.44855339999999 33.61567300000001, -84.44857620000001 33.6156428, -84.448489 33.6155747, -84.4485238 33.6155322, -84.4486258 33.615597, -84.4486472 33.61557689999999, -84.4487371 33.6155725, -84.4487438 33.6156071, -84.4487639 33.6156071))"
    result_geom = loads(result_str)
    reproj_geom = sol.utils.geo.reproject_geometry(input_str, input_crs=32616,
                                     target_crs=4326)
    area_sim = result_geom.intersection(reproj_geom).area/result_geom.area

    assert area_sim > 0.99999


def flatten_multichannel_mask():
    anarr = np.array([[[0, 0, 0, 1],
                       [0, 0, 1, 0],
                       [0, 0, 0, 1],
                       [0, 0, 0, 0]],
                      [[1, 1, 0, 0],
                       [1, 1, 1, 0],
                       [0, 0, 0, 0],
                       [0, 0, 0, 1]],
                      [[1, 0, 0, 1],
                       [0, 1, 0, 1],
                       [0, 1, 1, 0],
                       [0, 0, 0, 0]]], dtype='float')
    scaling_vector = [0.25, 1., 2.]
    result = sol.vector.mask.preds_to_binary(anarr, scaling_vector, bg_threshold=0.5)
    assert np.array_equal(result,
                          np.array([[255, 255, 0, 255],
                                    [255, 255, 255, 255],
                                    [0, 255, 255, 0],
                                    [0, 0, 0, 255]], dtype='uint8'))


def apply(input):
    flatten_multichannel_mask()
    reproject_from_wkt()
    rescale_auto()
    return "All tests passed"


def load():
    # Here you can optionally define a function that will be called when the algorithm is loaded.
    # The return object from this function can be passed directly as input to your apply function.
    # A great example would be any model files that need to be available to this algorithm
    # during runtime.
    # Any variables returned here, will be passed as the secondary argument to your 'algorithm' function

    # -- USAGE EXAMPLE ---
    # client = Algorithmia.client()
    # model_file_path = client.file('data://path/to/my/modelFile.hd5).getFile().name
    # keras_model = keras.load_model(model_path)
    # return keras_model

    return None

# This code turns your library code into an algorithm that can run on the platform.
# If you intend to use loading operations, remember to pass a `load` function as a second variable.
algo = Algorithmia.handler(apply, load)
# The 'serve()' function actually starts the algorithm, you can follow along in the source code
# to see how everything works.
algo.serve()
