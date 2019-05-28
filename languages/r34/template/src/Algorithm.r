library(algorithmia)
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

algorithm <- function(input) {
  paste("hello", input)
}

load <- function() {
  # Defines the loading function if defined
}

algo <- getAlgorithmHandler(algorithm, load)
algo$run()

