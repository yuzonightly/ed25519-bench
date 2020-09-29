package bench;

import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

// java -jar target/gs-maven-0.1.0.jar
public class Ed25519Benchmarks {
  public static void main(String[] args) throws RunnerException {
    Options opt = new OptionsBuilder().include(BouncyCastleBench.class.getSimpleName()).forks(1).build();
    new Runner(opt).run();
  }
}