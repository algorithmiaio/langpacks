library(algorithmia)
source("src/AlgorithmHandler.r")
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages
algorithmAdvanced <- function(input, context=NULL) {
    paste("hello", input)
}


loader <- function(){
  context <- list()
  client <- getAlgorithmiaClient(Sys.getenv("ALGORITHMIA_API_KEY"))
  context$local_file <- client$file("data://demo/collection/somefile.json")$getFile()
  context
}


setupAdvanced <- function(){
  Sys.setenv(ALGORITHMIA_API_KEY="simP1oZ9sfF7c1cBrYUQ08iBtdP1")
  algo <- getAlgorithmHandler()
  algo$setApplyFunction(algorithmAdvanced)
  algo$setOnLoadFunction(loader)
  algo$run()
}