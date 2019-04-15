AlgorithmHandler <- methods::setRefClass("AlgorithmHandler",
                                         fields = list(applyMethod = "function",
                                                       onLoadMethod = "function",
                                                       context = "list"),
                                         methods = list(
                                           setApplyFunction = function(func){
                                             applyMethod <<- func
                                           },
                                           setOnLoadFunction = function(func){
                                             onLoadMethod <<- func
                                           },
                                           run = function(){
                                             print("Not Implemented.")
                                           }
                                         ))


getAlgorithmHandler <- function(applyfunc=function(){}){
  AlgorithmHandler$new(applyMethod=applyfunc, onLoadMethod=function(){}, context=list())
}