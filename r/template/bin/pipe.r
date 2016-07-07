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

getResponse <- function(output) {
    if (typeof(output) == "raw") {
        list(result=base64enc::base64encode(output), metadata=list(content_type="binary"))
    } else {
        list(result=output, metadata=list(content_type="json"))
    }
}

while (TRUE) {
    line <- readLines(file("stdin"), n=1)
    if (is.null(line) | is.na(line) | nchar(line) == 0) {
        # We never seem to get here! TODO(james): fix this
        break
    }


    # TODO(james): do more error checking
    input <- rjson::fromJSON(line)
    inputData <- getInputData(input)
    output <- algorithm(inputData)

    # Flush stdout before writing back response
    flush.console()

    # TODO(james): more error checking around json serialization
    response = getResponse(output)
    writeLines(rjson::toJSON(response), con=fifo("/tmp/algoout", open="w", blocking=TRUE))
}
