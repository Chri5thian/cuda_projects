#include "wrapper.h"

extern "C" {
    void launch_add_kernel(int *a, int *b, int *c, int N);
}

void run_cuda(int *a, int *b, int *c, int N) {
    launch_add_kernel(a, b, c, N);
}
