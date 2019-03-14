library(testthat)

source("src/__ALGO__.r")
test_results <- test_dir("tests", reporter="summary")

for (i in 1:length(test_results)) {
  if (!is(test_results[[i]]$result[[1]], "expectation_success")) {
    stop("There were test failures")
  }
}
