import pickle
import urllib
import os
from Algorithmia.errors import AlgorithmException
from tempfile import mkdtemp

def download_image(client, image_url, size=None):
	"""
	Downloads the image file from the provided URL and returns the path to the locally saved file.
	NOTE: This method uses the util/SmartImageDownloader algorithm to perform the task and this implies
	that your algorithm should be able to call other algorithms and must have internet access.
	Parameters:
	client: Algorithmia python client
	image_url: Web url to the image that needs to be fetched
	dimensions: If provided, smart image downloader will attempt to format the image to fit the dimensions as a square, like 224x224
	"""
	if size:
		algo_input = {
			'image': image_url,
			'resize': {
				'width': size,
				'height': size
			},
		}
	else:
		algo_input = image_url
	img_data_path = client.algo("util/SmartImageDownloader").pipe(algo_input).result["savePath"][0]
	return client.file(img_data_path).getFile().name

def load_pickle_file(pickle_file_path, local_file = True, client=None):
	"""
	Download and load a pickle file into a python object. The file can exist in Algorithmia (data://) or any public url or locally
	Parameters:
	location: Absolute path to the file (remote or local)
	local_file (default=True): Optional argument to state whether the file is local or remote
	client (default=None): Optional argument for the algorithmia client needed to download if the file is in hosted in Algorithmia
	"""
	pickle_file = ''
	if local_file:
		pickle_file = pickle_file_path
	elif pickle_file_path.startswith('data://'):
		if client == None:
			raise AlgorithmException('Must specify a client object when attempting to fetch from data://', 'AlgorithmError')
		pickle_file = client.file(pickle_file_path).getFile().name
	else:
		pickle_file = 'data.pickle'
		urllib.URLopener().retrieve(pickle_file_path, pickle_file)
	with open(pickle_file, 'rb') as handle:
		loaded_classifier = pickle.load(handle)
	return loaded_classifier

def download_from_data_collection(algo_client, data_collection_path, local_path=None):
	"""
	Description:
		Downloads all files in a dataAPI collection, and downloads them locally.
		If no local path is given, a temporary one will be created.
	Parameters:
		algo_client: An algorithmia client for accessing the dataAPI
		data_collection_path: The dataAPI collection url
		local_path: The local path where the files will be downloaded. (optional)
	Returns:
		The local path, and a list of file names.
	"""
	file_names = []
	if local_path:
		if not os.path.exists(local_path):
			os.makedirs(local_path)
	else:
		local_path = mkdtemp()
	if not local_path.endswith("/"):
		local_path += "/"
	# For more info about Algorithmia's dataAPI: https://docs.algorithmia.com/#data-api-specification
	for data_file in algo_client.dir(data_collection_path).list():
		file_names.append(data_file.getName())
		os.rename(data_file.getFile().name, local_path + data_file.getName())

	return {"local_path": local_path, "file_names": file_names}


def type_check(dic, id, typedef):
	"""
	Description:
		checks the type of a field in a dictionary, if it's not one of the defined types, it throws an exception.
	Parameters:
	dic: The dictionary object that we should check against.
	id:  The specimen field name.
	typedef: The type or list of types that your field type should be a member of.
	Returns:
		The object contained in the dictionary at the objective field.
	"""
	if isinstance(typedef, type):
		if isinstance(dic[id], typedef):
			return dic[id]
		else:
			raise Exception("'{}' must be of {}".format(str(id), str(typedef)))
	else:
		for i in range(len(typedef)):
			if isinstance(dic[id], typedef[i]):
				return dic[id]
		raise Exception("'{}' must be of {}".format(str(id), str(typedef)))
