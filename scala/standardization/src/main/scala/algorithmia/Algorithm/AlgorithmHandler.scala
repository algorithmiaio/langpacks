package algorithmia.Algorithm


case class AlgorithmHandler[I1, I2, O](applyMethod: (I1, I2) => O, loadFunction: () => I2 = {throw new Exception("unimplemented")}){
  def setOnLoad(loadFunction: () => I2): AlgorithmHandler[I1, I2, O] ={
    AlgorithmHandler(this.applyMethod, loadFunction)
  }
  def run() {
    throw new Exception("unimplemented")
  }
}

