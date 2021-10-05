package bench;

import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;
import java.security.SecureRandom;
import java.security.SignatureException;
import java.util.concurrent.TimeUnit;

import net.i2p.crypto.eddsa.*;

import org.openjdk.jmh.annotations.*;

import java.security.InvalidKeyException;
import java.security.KeyPair;
import java.security.PrivateKey;
import java.security.PublicKey;

public class Ed25519Benchmarks {

  public static void main(String[] args) throws RunnerException, InvalidKeyException, SignatureException {
    Options optBouncy = new OptionsBuilder().include(BouncyCastleBench.class.getSimpleName()).forks(1).build();
    new Runner(optBouncy).run();
    Options optStr4d = new OptionsBuilder().include(Str4dBench.class.getSimpleName()).forks(1).build();
    new Runner(optStr4d).run();
    // SecureRandom r = new SecureRandom();
    // KeyPairGenerator keyGenerator = new KeyPairGenerator();
    // keyGenerator.generateKeyPair();

    // KeyPair keyPair = keyGenerator.generateKeyPair();
    // PrivateKey sk = keyPair.getPrivate();
    // PublicKey pk = keyPair.getPublic();

    // byte[] message = new byte[64];
    // r.nextBytes(message);

    // EdDSAEngine signer = new EdDSAEngine();
    // signer.initSign(sk);
    // signer.update(message, 0, message.length);
    // byte[] signature = signer.sign();
  }
}
