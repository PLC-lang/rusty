#include <stdbool.h>
#include <stdio.h>

unsigned long long SIEVE_SIZE = 500000000;
bool FLAGS[500000000] = {0};

void sieve() {
    unsigned long long i, j = 0;
    unsigned long long primes = 0;

    for (i = 2; i < SIEVE_SIZE; ++i) {
        if (FLAGS[i] == 0) {
            for (j = i * i; j < SIEVE_SIZE; j += i) {
                if (j < SIEVE_SIZE) {
                    FLAGS[j] = 1;
                }
            }
        }
    }

    for (i = 2; i < SIEVE_SIZE; ++i) {
        if (FLAGS[i] == 0) {
            primes += 1;
        }
    }

    printf("Primes found : %lld", primes);
}

int main(int argc, char **argv) {
    sieve();
}
