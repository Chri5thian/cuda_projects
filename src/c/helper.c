#include <stdio.h>

void print_array_helper_c(const int* array, int size) {
    for (int i = 0; i < size; ++i) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

void print_array_c(int *arr, int size) {
    printf("Array: ");
    print_array_helper_c(arr, size);
}
