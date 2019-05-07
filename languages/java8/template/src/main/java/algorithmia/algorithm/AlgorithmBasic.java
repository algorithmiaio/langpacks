package algorithmia.algorithm;
import com.algorithmia.algorithmHandler.*;
import com.algorithmia.*;

class AlgorithmBasic
{
    String Apply(String input) throws Exception{
        return "hello ".concat(input);
    }

    public static void main(String[] args) throws Exception{
        AlgorithmBasic defs = new AlgorithmBasic();
        AlgorithmHandler algo = new AlgorithmHandler<>(defs::Apply, String.class);
        algo.run();
    }
}
