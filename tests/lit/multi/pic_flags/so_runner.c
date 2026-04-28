#include <stdio.h>
#include <dlfcn.h>
#include <stdlib.h>

// Loads a shared library given as argv[1], looks up helper() and doubler(),
// calls them and prints the results.
int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <library.so>\n", argv[0]);
        return 1;
    }
    void *h = dlopen(argv[1], RTLD_NOW);
    if (!h) {
        fprintf(stderr, "dlopen: %s\n", dlerror());
        return 1;
    }
    int (*helper_fn)(void) = dlsym(h, "helper");
    int (*doubler_fn)(int) = dlsym(h, "doubler");
    if (!helper_fn || !doubler_fn) {
        fprintf(stderr, "dlsym: %s\n", dlerror());
        dlclose(h);
        return 1;
    }
    int result = helper_fn();
    printf("%d\n", result);
    printf("%d\n", doubler_fn(result));
    dlclose(h);
    return 0;
}
