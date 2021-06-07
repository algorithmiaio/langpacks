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


class AlgorithmError(Exception):
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return repr(self.value)

client = Algorithmia.client()


def runJob(file, name, collection):
    itype = getOutputFormat(file)
    # itype = getFiletype(file)
    oname = "/tmp/" + name
    otype = getOutputFormat(oname)
    print("input format: " + itype)
    print("output format: " + otype)
    result = ""
    if (otype == "pdf"):
        if (itype == "img"):
            pdf = imageToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        elif (itype == "html"):
            pdf = htmlToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        elif (itype == "csv"):
            pdf = csvToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        elif (itype == "doc"):
            pdf = docToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        elif (itype == "txt"):
            pdf = txtToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        elif (itype == "md"):
            pdf = mdToPDF(file, oname)
            result = saveFileToData(pdf, name, collection)
        else:
            raise AlgorithmError(
                "input format: " + itype + " not implemented yet for otype: " + otype + "!")

    # to img from X
    elif (otype == "img"):
        if (itype == "img"):
            img = imageToImage(file, oname)
            result = saveFileToData(img, name, collection)
        elif (itype == "pdf"):
            img = pdfToImage(file, oname)
            result = saveFileToData(img, name, collection)
        elif (itype == "html"):
            img = htmlToImage(file, oname)
            result = saveFileToData(img, name, collection)
        elif (itype == "doc"):
            img = docToImage(file, oname)
            result = saveFileToData(img, name, collection)
        else:
            raise AlgorithmError("input format: " + itype + " not implemented yet for otype: " + otype + "!")

    # to doc from X
    elif (otype == 'doc'):
        if (itype == "html"):
            doc = htmlToDoc(file, oname)
            result = saveFileToData(doc, name, collection)
        elif (itype == "md"):
            doc = mdToDoc(file, oname)
            result = saveFileToData(doc, name, collection)
        elif (itype == "txt"):
            doc = txtToDoc(file, oname)
            result = saveFileToData(doc, name, collection)
        else:
            raise AlgorithmError("input format: " + itype + " not implemented yet for otype: " + otype + "!")
    # to html from X
    elif (otype == 'html'):
        if (itype == 'txt'):
            html = txtToHtml(file, oname)
            result = saveFileToData(html, name, collection)
        elif (itype == 'doc'):
            html = docToHtml(file, oname)
            result = saveFileToData(html, name, collection)
        elif (itype == 'md'):
            html = mdToHtml(file, oname)
            result = saveFileToData(html, name, collection)
        else:
            raise AlgorithmError(
                "input format: " + itype + " not implemented yet for otype: " + otype + "!")
    elif (otype == 'md'):
        if (itype == 'doc'):
            md = docToMD(file, oname)
            result = saveFileToData(md, name, collection)
        elif (itype == 'txt'):
            md = txtToMD(file, oname)
            result = saveFileToData(md, name, collection)
        else:
            raise AlgorithmError(
                "input format: " + itype + " not implemented yet for otype: " + otype + "!")
    else:
        raise AlgorithmError("output format: " + otype + " not implemented yet!")
    cleanUp(oname)
    return result


# ------------------------------------------
# TO PDFS
def htmlToPDF(html, outname):
    out = subprocess.check_output((wkhtmltox + '/wkhtmltopdf', '--quiet', html, outname))
    print(out)
    return outname


def imageToPDF(img, outname):
    pdf = FPDF()
    pdf.add_page()
    pdf.image(img, 10, 10, 0, 0)
    pdf.output(outname, 'F')
    return outname


def csvToPDF(csvFile, outname):
    try:
        with open(csvFile, 'rb') as csvIterable:
            reader = csv.reader(csvIterable, dialect='excel')
            columns = reader.next()
            pt = PrettyTable(columns)
            pt.align[columns[0]] = "l"
            pt.padding_width = 1
            for row in reader:
                pt.add_row(row)
            lines = pt.get_string()
            line = lines.split('\n')[0]
            numLines = len(lines.split('\n'))
            pdf = FPDF()
            pdf.set_line_width(0)
            pdf.add_page()
            pdf.set_font('Arial', '', 8)
            pdf.multi_cell(w=0, h=3, txt=lines, border=0, align="L")
            pdf.output(outname, 'F')
            return outname
    except:
        docx = mdToDoc(csvFile, outname + '.docx')
        docToPDF(docx, outname)
        return outname


# since this uses pandoc, it can take any document-like input, not just .doc
def docToPDF(data, outname):
    # html = docToHtml(data, outname+".html")
    # return htmlToPDF(html, outname)
    generalPandoc(data, outname)
    return outname


def txtToPDF(data, outname):
    generalPandoc(data, outname)
    return outname


def mdToPDF(data, outname):
    generalPandoc(data, outname)
    return outname


# -----------------------------------------
# TO IMAGES
def imageToImage(data, outname):
    img = Image.open(data)
    img.convert('RGB').save(outname)
    return outname


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

    # print("data: "+data)
    # print("output_name: "+out_path)
    # img =  Magic(filename=data, resolution=300)
    # with open(out_path, 'w') as outFile:
    #     img.save(file=outFile)
    #     return out_path


def htmlToImage(html, outName):
    out = subprocess.check_output((wkhtmltox + '/wkhtmltoimage', '--quiet', html, outName))
    print(out)
    return outName


def txtToImage(data, outname):
    pdf = txtToPDF(data, "txt.pdf")
    img = pdfToImage(pdf, outname)
    return outname


def docToImage(data, outname):
    html = docToHtml(data, "doc.html")
    img = htmlToImage(html, outname)
    return outname


# --------------------------------------
# TO DOC
def htmlToDoc(data, outName):
    generalPandoc(data, outName)
    return outName


def txtToDoc(data, outName):
    generalPandoc(data, outName)
    return outName


def mdToDoc(data, outname):
    generalPandoc(data, outname)
    return outname


# -------------------------------------
# TO HTML
def txtToHtml(data, outname):
    generalPandoc(data, outname)
    return outname


def docToHtml(data, outname):
    return generalPandoc(data, outname)


def mdToHtml(data, outname):
    return generalPandoc(data, outname)


# ------------------------------------
# TO MD

def docToMD(data, outname):
    generalPandoc(data, outname)
    return outname


def txtToMD(data, outname):
    generalPandoc(data, outname)
    return outname


def generalPandoc(input, output):
    command = ["pandoc", "-s", input, "-o", output]
    proc = subprocess.Popen(command, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    out, err = proc.communicate()
    proc.wait()
    if err == '':
        print(out)
        return output
    else:
        raise AlgorithmError("pandoc failed: " + err)


def downloadFile(urlData):
    if isinstance(urlData, string_types):
        # if not os.path.isdir("/tmp/files"):
        dir = "/tmp/files/"
        try:
            os.mkdir("/tmp/files")
        except:
            pass
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
    elif isinstance(urlData, bytearray):
        return Image.open(io.BytesIO(urlData))


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


def getOutputFormat(name):
    ext = getExtension(name)
    if ext == ".jpg" or ext == ".png" or ext == ".gif" or ext == ".tiff" or ext == ".bmp":
        return "img"
    elif ext == ".pdf":
        return "pdf"
    elif ext == ".docx" or ext == ".odt":
        return "doc"
    elif ext == ".md":
        return "md"
    elif ext == ".html":
        return "html"
    elif ext == ".txt":
        return "txt"
    else:
        return "none"


def getFileName(path):
    if (path.startswith("http://") or path.startswith("https://")):
        path = urllib.parse.urlsplit(path).path
        filename = posixpath.basename(path)
        return filename
    else:
        filename = os.path.basename(path)
        return filename


def getWkhtmltoX():
    if (os.path.exists(whp_bin_path)):
        return whp_bin_path
    else:
        local = client.file(whp_dataPath).getFile().name
        proc = subprocess.Popen(['tar', '-xf', local, '-C', '/tmp'])
        if (proc.wait() == 0):
            return whp_bin_path
        else:
            raise AlgorithmError("couldn't get the wkhtmltopdf binaries.")


def getExtension(fileName):
    return os.path.splitext(fileName)[1]


def checkCSV(filename):
    try:
        with open(filename) as csvfile:
            reader = csv.DictReader(csvfile)
            k = 0
            for row in reader:
                k = k + 1
            if k > 10:
                return True
            else:
                return False
    except csv.Error:
        # File appears not to be in CSV format; move along
        return False


def apply(json):
    if isinstance(json, dict):
        if "collection" in json and "input" in json and "output" in json:
            collection = json["collection"]
            input = json["input"]
            output = json["output"]
            file = downloadFile(input)
            out = runJob(file, output, collection)
            return out
        else:
            raise AlgorithmError("algorithm input must contain: \"input\", \"collection\", and \"output\"")
    else:
        raise AlgorithmError("input must be json.")