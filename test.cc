
typedef struct {
    int c[];
} B;

typedef struct {
    B b;
} A;

A a;


int main(int argc, char** argv) {
    return a.b.c[3].a.b;
}