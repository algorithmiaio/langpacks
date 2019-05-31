library(algorithmia)
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

algorithm <- function(input) {
  paste("hello", input)
}

load <- function() {
  # Here you can optionally define a function called when the algorithm is loaded
}

algo <- getAlgorithmHandler(algorithm, load)
algo$serve()

