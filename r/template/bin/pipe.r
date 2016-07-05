# TODO(james): push this to setup
install.packages("jsonlite")
library(jsonlite)

conf <- fromJSON("algorithmia.conf")
source(sprintf("src/%s.r", conf$algoname))

print("PIPE_INIT_COMPLETE")
flush.console()

out <- fifo("/tmp/algoout", open="w", blocking=TRUE)

while (TRUE) {
    line <- readLines(file("stdin"), n=1)
    if (is.null(line) | is.na(line) | nchar(line) == 0) {
        # We never seem to get here! TODO(james): fix this
        break
    }

    # TODO(james): do more error checking
    input <- fromJSON(line)
    response <- algorithm(input)

    # Flush stdout before writing back response
    flush.console()
    writeLines(toJSON(response), con=out)
}
