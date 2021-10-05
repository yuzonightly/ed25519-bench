# Benchmarks from the warner/python-pure25519 repository.
# With a few modifications.


import timeit


class PycaBench:

    def __init__(self):
        pass

    def do(self, setup_statements, statement):
        # extracted from timeit.py
        t = timeit.Timer(stmt=statement,
                         setup="\n".join(setup_statements))
        # determine number so that 1.0 <= total time < 10.0
        for i in range(1, 10):
            number = 10**i
            x = t.timeit(number)
            if x >= 1.0:
                break
        return x / number

    def abbrev(self, t):
        if t > 1.0:
            return "%.3fs" % t
        if t > 1e-3:
            return "%.2fms" % (t*1e3)
        return "%.2fus" % (t*1e6)

    # We are only interested in the mininum value in the benchmark vector.
    def p(self, name, setup_statements, statements):
        t = sorted([self.do(setup_statements, statements) for i in range(3)])
        # print("%12s: %s (%s)" % (name,
        #                          self.abbrev(min(t)),
        #                          " ".join([self.abbrev(s) for s in t])))
        print("%12s: %s (%s)" % (name,
                                 t,
                                 " ".join([self.abbrev(s) for s in t])))

    def bench(self):
        S1 = "from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey"
        S2 = "msg=b''"
        S3 = "private_key = Ed25519PrivateKey.generate()"
        S4 = "signature = private_key.sign(msg)"
        S5 = "public_key = private_key.public_key()"
        S6 = "public_key.verify(signature, msg)"

        print("pyca benchmarks (in seconds)")
        self.p("Keypair generation", [S1, S2], S3)
        self.p("Signature generation", [S1, S2, S3], S4)
        self.p("Signature verification", [S1, S2, S3, S4, S5], S6)
