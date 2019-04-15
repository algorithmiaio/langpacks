source("src/AlgorithmBasic.r")
source("src/AlgorithmAdvanced.r")

main = function(){
  algo = setupBasic()
  algo$run()
}
main()