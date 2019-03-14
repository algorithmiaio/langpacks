library("base64enc")
library("rjson")

conf <- rjson::fromJSON(file="algorithmia.conf")
source(sprintf("src/%s.r", conf$algoname))

print("PIPE_INIT_COMPLETE")
flush.console()

getInputData <- function(input) {
    if (input$content_type == "binary") {
        base64enc::base64decode(input$data)
    } else {
        input$data
    }
}

getResponseObject <- function(output) {
    if (typeof(output) == "raw") {
        list(result=base64enc::base64encode(output), metadata=list(content_type="binary"))
    } else if (is.character(output) & length(output) == 1) {
        list(result=output, metadata=list(content_type="text"))
    } else {
        list(result=output, metadata=list(content_type="json"))
    }
}

getResponseAsJsonString <- function(output) {
    tryCatch({
        rjson::toJSON(output)
    },
    error = function(e) {
        print(paste0("Error in getResponse: ", e))
        rjson::toJSON(list(error=list(message=toString(e), stacktrace="pipe.r:getResponseAsJsonString", error_type="AlgorithmError")))
    })
}


outputFile <- fifo("/tmp/algoout", blocking=TRUE)
inputFile <- file("stdin")
open(inputFile)

while (length(line <- readLines(inputFile, n=1)) > 0) {
    stage <- "parsing"
    output <- tryCatch({
        input <- rjson::fromJSON(line)
        inputData <- getInputData(input)
        stage <- "algorithm"
        getResponseObject(algorithm(inputData))
    },
    error = function(e) {
        list(error=list(message=toString(e), stacktrace=stage, error_type="AlgorithmError"))
    })

    # Flush stdout before writing back response
    flush.console()

    response = getResponseAsJsonString(output)
    writeLines(response, con=outputFile)
}
