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

getResponse <- function(output) {
    tryCatch({
        rjson::toJSON(getResponseObject(output))
    },
    error = function(e) {
        print(paste0("Error in getResponse: ", e))
        rjson::toJSON(list(error=list(message=toString(e), stacktrace="pipe.r:getResponse", error_type="AlgorithmError")))
    },
    warning = function(w) {
        print(paste0("Warning in getResponse: ", w))
        rjson::toJSON(getResponseObject(output))
    })
}

parseLine <- function(line) {
    tryCatch({
        rjson::fromJSON(line)
    },
    error = function(e) {
        print(paste0("Error in getInputData: ", e))
        NA
    },
    warning = function(w) {
        print(paste0("Warning in getInputData: ", w))
        rjson::fromJSON(line)
    })
}

outputFile <- fifo("/tmp/algoout", open="w", blocking=TRUE)

while (TRUE) {
    line <- readLines(file("stdin"), n=1)
    if (is.null(line) | is.na(line) | nchar(line) == 0) {
        # We never seem to get here! TODO(james): fix this
        break
    }


    input <- parseLine(line)
    output <- if (is.na(input)) {
        list(error=list(message="Could not parse json input", stacktrace="pipe.r:parseLine", error_type="AlgorithmError"))
    } else {
        inputData <- getInputData(input)

        tryCatch({
            algorithm(inputData)
        },
        error = function(e) {
            list(error=list(message=toString(e), stacktrace="algorithm", error_type="AlgorithmError"))
        })
    }

    # Flush stdout before writing back response
    flush.console()

    response = getResponse(output)
    writeLines(response, con=outputFile)
}
