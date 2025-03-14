use libc::c_int;

unsafe extern "C" {
    fn run_cuda(a: *mut c_int, b: *mut c_int, c: *mut c_int, n: c_int);
    fn print_array(array: *const c_int, size: c_int);
    fn create_array_manipulator(array: *mut c_int, size: c_int) -> *mut ArrayManipulator;
    fn destroy_array_manipulator(manipulator: *mut ArrayManipulator);
    fn array_manipulator_increment_elements(manipulator: *mut ArrayManipulator, value: c_int);
    fn array_manipulator_print(manipulator: *const ArrayManipulator);

    // C functions
    fn print_array_helper_c(array: *const c_int, size: c_int);
    fn print_array_c(arr: *mut c_int, size: c_int);
}

#[repr(C)]
struct ArrayManipulator {
    _private: [u8; 0], // Add a dummy field to make the struct FFI-safe
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    let mut b = vec![6, 7, 8, 9, 10];
    let mut c = vec![0; 5];
    let n = a.len() as c_int;

    unsafe {
        run_cuda(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), n);
        print_array(c.as_ptr(), n);

        let manipulator = create_array_manipulator(c.as_mut_ptr(), n);
        array_manipulator_increment_elements(manipulator, 10);
        array_manipulator_print(manipulator);
        destroy_array_manipulator(manipulator);

        // Using C functions
        print_array_helper_c(c.as_ptr(), n);
        print_array_c(c.as_mut_ptr(), n);
    }
}
