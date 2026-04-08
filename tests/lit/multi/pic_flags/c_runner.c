#include <stdio.h>

// Declarations matching the ST functions exported from the shared library
extern int helper(void);
extern int doubler(int x);

int main(void) {
    int result = helper();
    printf("%d\n", result);
    printf("%d\n", doubler(result));
    return 0;
}
