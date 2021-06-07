import Algorithmia
from fpdf import FPDF
from PIL import Image
import shutil
from prettytable import PrettyTable
from wand.image import Image as Magic
import subprocess
import posixpath
import urllib
from six import string_types
import io, csv, os, sys, re


"""
Example payload:
{"input":"http://www.africau.edu/images/default/sample.pdf", "output":"sample.png", "collection":"data://.algo/temp"}
"""

class AlgorithmError(Exception):
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return repr(self.value)


client = Algorithmia.client()


# signature from http://stackoverflow.com/questions/23706661/imagemagick-wand-save-pdf-pages-as-images
def pdfToImage(data, out_path):
    extension = getExtension(out_path).split(".")[1]
    with open(data, 'rb') as f:
        with Magic(blob=f) as pdf:
            pages = len(pdf.sequence)
            print("pdf pages: {}\n pdf width: {}\n pdf height: {}".format(pages, pdf.width, pdf.height))
            image = Magic(width=pdf.width + int(pdf.width / 2), height=pdf.height * pages)
            for i in range(pages):
                image.composite(
                    pdf.sequence[i],
                    top=pdf.height * i,
                    left=0)
            image.make_blob(extension)
            image.save(filename=out_path)
    return out_path


def downloadFile(urlData):
    if isinstance(urlData, string_types):
        # if not os.path.isdir("/tmp/files"):
        dir = "/tmp/"
        if urlData.startswith("http://") or urlData.startswith("https://"):
            (response, _) = urllib.request.urlretrieve(urlData, dir + getFileName(urlData))
            noSpaces = response.replace(' ', '-')
            os.rename(response, noSpaces)
            return noSpaces
        if re.search("[\\w\\+]+://.+", urlData):
            dataf = client.file(urlData)
            noSpaces = dir + dataf.getName().replace(' ', '-')
            response = dataf.getFile().name
            os.rename(response, noSpaces)
            return noSpaces
        else:
            raise AlgorithmError("Please provide a valid data://, http:// or https:// URL.")
    else:
        raise AlgorithmError("input must be a URL or URI")


def saveFileToData(local, outName, collection):
    out = collection + '/' + outName
    print(out)
    print(local)
    client.file(out).putFile(local)
    return out


def cleanUp(output_path):
    shutil.rmtree('/tmp/files')
    os.remove(output_path)


def getEncoding(file):
    proc = subprocess.Popen(['file', '-bi', file], stdout=subprocess.PIPE)
    out, _ = proc.communicate()
    breaking = out.split("; ")[1]
    body = breaking.split("=")[1]
    print(body)
    return body


def getFileName(path):
    if path.startswith("http://") or path.startswith("https://"):
        path = urllib.parse.urlsplit(path).path
        filename = posixpath.basename(path)
        return filename
    else:
        filename = os.path.basename(path)
        return filename


def getExtension(fileName):
    return os.path.splitext(fileName)[1]


def apply(json):
    if isinstance(json, dict):
        if "collection" in json and "input" in json and "output" in json:
            collection = json["collection"]
            input = json["input"]
            o_filename = json["output"]
            file = downloadFile(input)
            local_output_path = "/tmp/" + o_filename
            pdf = imageToPDF(file, local_output_path)
            result = saveFileToData(pdf, o_filename, collection)
            return result
        else:
            raise AlgorithmError("algorithm input must contain: \"input\", \"collection\", and \"output\"")
    else:
        raise AlgorithmError("input must be json.")
