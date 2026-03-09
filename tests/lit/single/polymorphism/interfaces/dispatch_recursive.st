// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface method implementation dispatches through another interface reference passed as an
// argument, forming a chain of virtual calls. Additionally, a countdown pattern is used where an FB calls
// back into itself through an interface reference, testing re-entrant dispatch.

INTERFACE ICounter
    METHOD countdown
        VAR_INPUT
            n: DINT;
            partner: ICounter;
        END_VAR
    END_METHOD
END_INTERFACE

// FbEven prints on even numbers and delegates the next step to its partner
FUNCTION_BLOCK FbEven IMPLEMENTS ICounter
    METHOD countdown
        VAR_INPUT
            n: DINT;
            partner: ICounter;
        END_VAR

        IF n <= 0 THEN
            printf('done$N');
            RETURN;
        END_IF;

        printf('Even::%d$N', n);
        partner.countdown(n - 1, partner);
    END_METHOD
END_FUNCTION_BLOCK

// FbOdd prints on odd numbers and delegates the next step to its partner
FUNCTION_BLOCK FbOdd IMPLEMENTS ICounter
    METHOD countdown
        VAR_INPUT
            n: DINT;
            partner: ICounter;
        END_VAR

        IF n <= 0 THEN
            printf('done$N');
            RETURN;
        END_IF;

        printf('Odd::%d$N', n);
        partner.countdown(n - 1, partner);
    END_METHOD
END_FUNCTION_BLOCK

// FbSelf always delegates back to itself through the interface, testing re-entrant dispatch
FUNCTION_BLOCK FbSelf IMPLEMENTS ICounter
    METHOD countdown
        VAR_INPUT
            n: DINT;
            partner: ICounter;
        END_VAR

        IF n <= 0 THEN
            printf('done$N');
            RETURN;
        END_IF;

        printf('Self::%d$N', n);
        partner.countdown(n - 1, partner);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        even: FbEven;
        odd: FbOdd;
        self: FbSelf;
        refEven: ICounter;
        refOdd: ICounter;
        refSelf: ICounter;
    END_VAR

    refEven := even;
    refOdd := odd;
    refSelf := self;

    // Alternating dispatch: Even starts, delegates to Odd, which delegates to itself, etc.
    // CHECK: Even::3
    // CHECK-NEXT: Odd::2
    // CHECK-NEXT: Odd::1
    // CHECK-NEXT: done
    refEven.countdown(3, refOdd);

    // Self-referential: FbSelf dispatches back into itself through the interface
    // CHECK-NEXT: Self::3
    // CHECK-NEXT: Self::2
    // CHECK-NEXT: Self::1
    // CHECK-NEXT: done
    refSelf.countdown(3, refSelf);
END_FUNCTION
