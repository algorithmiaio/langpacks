library(algorithmia)
# API calls will begin at the algorithm() method, with the request body passed as 'input'
# For more details, see algorithmia.com/developers/algorithm-development/languages

algorithm <- function(input) {
  paste("hello", input)
}

# Here is an example of an advanced form of an algorithm function, using a load function (like the example below)
# -- ADVANCED ALGORITHM USAGE --
#   algorithm <- function(input, keras_model) {
#   ....
#   prediction <- keras_model.predict(input)
#   result <- list(class <- prediction[0], confidence <- prediction[1])
#   result
# }


load <- function() {
  # Here you can optionally define a function called when the algorithm is loaded.
  # A great example would be any model files that need to be available to this algorithm
  # during runtime.
  # Any variables returned here, will be passed as the secondary argument to your 'algorithm' function
  
  # -- USAGE EXAMPLE ---
  # client <- getAlgorithmiaClient()
  # model_file_path <- client$file('data://path/to/my/modelFile.hd5)$getFile()$name
  # keras_model <- keras.load_model(model_path)
  # keras_model
  NULL
}

# This code turns your library code into an algorithm that can run on the platform.
# If you intend to use loading operations, remember to pass a `load` function as a second variable.
algo <- getAlgorithmHandler(algorithm, load)
# The 'serve()' function actually starts the algorithm, you can follow along in the source code.
algo$serve()
