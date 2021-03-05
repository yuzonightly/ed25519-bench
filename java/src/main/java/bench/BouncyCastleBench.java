package bench;

import java.security.SecureRandom;
import java.util.concurrent.TimeUnit;

import org.bouncycastle.crypto.AsymmetricCipherKeyPair;
import org.bouncycastle.crypto.CryptoException;
import org.bouncycastle.crypto.Signer;
import org.bouncycastle.crypto.generators.Ed25519KeyPairGenerator;
import org.bouncycastle.crypto.params.AsymmetricKeyParameter;
import org.bouncycastle.crypto.params.Ed25519KeyGenerationParameters;
import org.bouncycastle.crypto.signers.Ed25519Signer;

import org.openjdk.jmh.annotations.*;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.NANOSECONDS)
@Warmup(iterations = 5000, time = 1, timeUnit = TimeUnit.NANOSECONDS)
@Measurement(iterations = 50000, time = 2, timeUnit = TimeUnit.NANOSECONDS)
@Fork(1)
@State(Scope.Benchmark)

public class BouncyCastleBench {
    public Ed25519KeyPairGenerator keyGenerator;
    public AsymmetricKeyParameter sk;
    public AsymmetricKeyParameter pk;
    public byte[] message;
    public byte[] signature;

    @Setup
    public void setup() throws CryptoException {
        SecureRandom r = new SecureRandom();
        this.keyGenerator = new Ed25519KeyPairGenerator();
        this.keyGenerator.init(new Ed25519KeyGenerationParameters(r));

        AsymmetricCipherKeyPair keyPair = this.keyGenerator.generateKeyPair();
        this.sk = keyPair.getPrivate();
        this.pk = keyPair.getPublic();

        this.message = new byte[64];
        r.nextBytes(this.message);

        Signer signer = new Ed25519Signer();
        signer.init(true, this.sk);
        signer.update(this.message, 0, this.message.length);
        this.signature = signer.generateSignature();
    }

    @Benchmark
    public AsymmetricCipherKeyPair keyGeneration() {
        return this.keyGenerator.generateKeyPair();
    }

    @Benchmark
    public byte[] signatureGeneration() throws CryptoException {
        Signer signer = new Ed25519Signer();
        signer.init(true, this.sk);
        signer.update(this.message, 0, this.message.length);
        return signer.generateSignature();
    }

    @Benchmark
    public boolean verification() {
        Signer verifier = new Ed25519Signer();
        verifier.init(false, this.pk);
        verifier.update(this.message, 0, this.message.length);
        return verifier.verifySignature(this.signature);
    }
}
