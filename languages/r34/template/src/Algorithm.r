library(algorithmia)
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

algorithm <- function(input) {
  paste("hello", input)
}

algo <- getAlgorithmHandler(algorithm)
algo$run()


# For a more advanced example with loading, see the example below.
'algorithmAdvanced <- function(input, context=NULL) {
    paste("hello", input)
}


loader <- function(){
  context <- list()
  client <- getAlgorithmiaClient()
  context$local_file <- client$file("data://demo/collection/somefile.json")$getFile()
  context
}


setupAdvanced <- function(){
  Sys.setenv(ALGORITHMIA_API_KEY="API_KEY")
  algo <- getAlgorithmHandler()
  algo$setApplyFunction(algorithmAdvanced)
  algo$setOnLoadFunction(loader)
  algo$run()
}'
