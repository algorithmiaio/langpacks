# TODO(james): push this to setup
install.packages("rjson")
library(rjson)

conf <- rjson::fromJSON(file="algorithmia.conf")
source(sprintf("src/%s.r", conf$algoname))

print("PIPE_INIT_COMPLETE")
flush.console()

while (TRUE) {
    line <- readLines(file("stdin"), n=1)
    if (is.null(line) | is.na(line) | nchar(line) == 0) {
        # We never seem to get here! TODO(james): fix this
        break
    }

    # TODO(james): do more error checking
    input <- rjson::fromJSON(line)
    output <- algorithm(input)

    # Flush stdout before writing back response
    flush.console()

    # TODO(james): change content type, maybe? and more error checking around json serialization
    response = list(result=output, metadata=list(content_type="json"))
    writeLines(rjson::toJSON(response), con=fifo("/tmp/algoout", open="w", blocking=TRUE))
}
