#include <cuda_runtime.h>
#include <stdio.h>

__global__ void addKernel(int *d_a, int *d_b, int *d_c, int N) {
    int i = threadIdx.x;
    if (i < N) {
        d_c[i] = d_a[i] + d_b[i];
    }
}

extern "C" void launch_add_kernel(int *a, int *b, int *c, int N) {
    int *d_a, *d_b, *d_c;
    cudaMalloc((void **)&d_a, N * sizeof(int));
    cudaMalloc((void **)&d_b, N * sizeof(int));
    cudaMalloc((void **)&d_c, N * sizeof(int));

    cudaMemcpy(d_a, a, N * sizeof(int), cudaMemcpyHostToDevice);
    cudaMemcpy(d_b, b, N * sizeof(int), cudaMemcpyHostToDevice);

    addKernel<<<1, N>>>(d_a, d_b, d_c, N);
    cudaMemcpy(c, d_c, N * sizeof(int), cudaMemcpyDeviceToHost);

    cudaFree(d_a);
    cudaFree(d_b);
    cudaFree(d_c);
}