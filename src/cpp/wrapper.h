#ifndef WRAPPER_H
#define WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

void run_cuda(int *a, int *b, int *c, int N);

#ifdef __cplusplus
}
#endif

#endif
