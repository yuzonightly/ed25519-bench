package bench;

import java.security.SecureRandom;
import java.security.SignatureException;
import java.util.concurrent.TimeUnit;

import net.i2p.crypto.eddsa.*;

import org.openjdk.jmh.annotations.*;

import java.security.InvalidKeyException;
import java.security.KeyPair;
import java.security.PrivateKey;
import java.security.PublicKey;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.MICROSECONDS)
@Warmup(iterations = 50000, time = 1, timeUnit = TimeUnit.MICROSECONDS)
@Measurement(iterations = 100000, time = 2, timeUnit = TimeUnit.MICROSECONDS)
@Fork(1)
@State(Scope.Benchmark)

public class Str4dBench {
    public KeyPairGenerator keyGenerator;
    public PrivateKey sk;
    public PublicKey pk;
    public byte[] message;
    public byte[] signature;

    @Setup
    public void setup() throws InvalidKeyException, SignatureException {
        SecureRandom r = new SecureRandom();
        this.keyGenerator = new KeyPairGenerator();
        this.keyGenerator.generateKeyPair();

        KeyPair keyPair = this.keyGenerator.generateKeyPair();
        this.sk = keyPair.getPrivate();
        this.pk = keyPair.getPublic();

        this.message = new byte[0];
        r.nextBytes(this.message);

        EdDSAEngine signer = new EdDSAEngine();
        signer.initSign(sk);
        signer.update(this.message, 0, this.message.length);
        this.signature = signer.sign();
    }

    @Benchmark
    public KeyPair keyGeneration() {
        return this.keyGenerator.generateKeyPair();
    }

    @Benchmark
    public byte[] signatureGeneration() throws SignatureException, InvalidKeyException {
        EdDSAEngine signer = new EdDSAEngine();
        signer.initSign(this.sk);
        signer.update(this.message, 0, this.message.length);
        return signer.sign();
    }

    @Benchmark
    public boolean verification() throws SignatureException, InvalidKeyException {
        EdDSAEngine verifier = new EdDSAEngine();
        verifier.initVerify(this.pk);
        verifier.update(this.message, 0, this.message.length);
        return verifier.verify(this.signature);
    }
}
