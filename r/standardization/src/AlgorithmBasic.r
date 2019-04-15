library(algorithmia)
source("src/AlgorithmHandler.r")
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
algorithmBasic <- function(input, context=NULL) {
  paste("hello", input)
}

setupBasic <- function(){
  algo <- getAlgorithmHandler(algorithmBasic)
  algo
}