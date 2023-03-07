VAR_GLOBAL
    flags : ARRAY[1..500_000_000] OF BOOL;
    sieve_size : ULINT := 500_000_000;
END_VAR

// en.wikipedia.org/wiki/Sieve_of_Eratosthenes
FUNCTION sieve : DINT
    VAR
        i, j : ULINT;
        primes : ULINT;
    END_VAR

    FOR i := 2 TO sieve_size DO
        IF flags[i] = 0 THEN
            FOR j := i * i TO sieve_size BY i DO
                IF j < sieve_size THEN
                    flags[j] := 1;
                END_IF;
            END_FOR;
        END_IF;
    END_FOR;

    FOR i := 2 TO sieve_size DO
        IF flags[i] = 0 THEN
            primes := primes + 1;
        END_IF;
    END_FOR;

    printf('Primes found: %lld (correct: %d)$N', primes, (primes - 1) = 26_355_867);
END_FUNCTION

FUNCTION main : DINT
    sieve();
END_FUNCTION

{external}
FUNCTION printf : DINT
    VAR_INPUT {ref} format : STRING; END_VAR
    VAR_INPUT args : ...; END_VAR
END_FUNCTION