#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "ed25519-donna/ed25519.h"

int main()
{
    const unsigned char message[] = "";
    const int message_len = strlen((char *)message);

    ed25519_secret_key sk = {0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae, 0x7f, 0x60};

    ed25519_public_key pk;

    ed25519_signature sig;

    clock_t start;
    clock_t end;
    int i;

    printf("testing key generation performance: ");
    start = clock();
    for (i = 0; i < 10000; ++i)
    {
        ed25519_publickey(sk, pk);
    }
    end = clock();

    printf("%fus per keypair\n", ((double)((end - start) * 1000)) / CLOCKS_PER_SEC / i * 1000);

    printf("testing sign performance: ");
    start = clock();
    for (i = 0; i < 10000; ++i)
    {
        ed25519_sign(message, message_len, sk, pk, sig);
    }
    end = clock();

    printf("%fus per signature\n", ((double)((end - start) * 1000)) / CLOCKS_PER_SEC / i * 1000);

    printf("testing verify performance: ");
    start = clock();
    for (i = 0; i < 10000; ++i)
    {
        ed25519_sign_open(message, message_len, pk, sig);
    }
    end = clock();

    printf("%fus per signature\n", ((double)((end - start) * 1000)) / CLOCKS_PER_SEC / i * 1000);

    return 0;
}
