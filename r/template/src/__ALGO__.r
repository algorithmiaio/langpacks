library(algorithmia)

# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
algorithm <- function(input) {
    paste("hello", input)
}
