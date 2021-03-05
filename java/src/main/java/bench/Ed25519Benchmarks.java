package bench;

import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

// java -jar target/gs-maven-0.1.0.jar

// REMEMBER: The numbers below are just data. To gain reusable insights, you need to follow up on
// why the numbers are the way they are. Use profilers (see -prof, -lprof), design factorial
// experiments, perform baseline and negative tests that provide experimental control, make sure
// the benchmarking environment is safe on JVM/OS/HW level, ask for reviews from the domain experts.
// Do not assume the numbers tell you what you want them to tell.

// Benchmarks + Theory Chapter + Documentation + Finish Code.

public class Ed25519Benchmarks {
  public static void main(String[] args) throws RunnerException {
    // Options opt = new
    // OptionsBuilder().include(BouncyCastleBench.class.getSimpleName()).forks(1).build();
    // new Runner(opt).run();
    Options opt = new OptionsBuilder().include(Str4dBench.class.getSimpleName()).forks(1).build();
    new Runner(opt).run();
  }
}
