#include "wrapper.h"
#include <iostream>

extern "C" {
    void launch_add_kernel(int *a, int *b, int *c, int N);
}

void run_cuda(int *a, int *b, int *c, int N) {
    launch_add_kernel(a, b, c, N);
}

// Example helper function
extern "C" void print_array(const int* array, int size) {
    for (int i = 0; i < size; ++i) {
        std::cout << array[i] << " ";
    }
    std::cout << std::endl;
}

// Example class
class ArrayManipulator {
public:
    ArrayManipulator(int* array, int size) : array_(array), size_(size) {}

    void increment_elements(int value) {
        for (int i = 0; i < size_; ++i) {
            array_[i] += value;
        }
    }

    void print() const {
        print_array(array_, size_);
    }

private:
    int* array_;
    int size_;
};

// C-compatible functions to interact with ArrayManipulator
extern "C" ArrayManipulator* create_array_manipulator(int* array, int size) {
    return new ArrayManipulator(array, size);
}

extern "C" void destroy_array_manipulator(ArrayManipulator* manipulator) {
    delete manipulator;
}

extern "C" void array_manipulator_increment_elements(ArrayManipulator* manipulator, int value) {
    manipulator->increment_elements(value);
}

extern "C" void array_manipulator_print(const ArrayManipulator* manipulator) {
    manipulator->print();
}
