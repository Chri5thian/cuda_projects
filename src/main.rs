use std::os::raw::c_int;

#[link(name = "cuda_wrapper")]
unsafe extern "C" {
    fn run_cuda(a: *mut c_int, b: *mut c_int, c: *mut c_int, N: c_int);
    fn print_array(arr: *mut c_int, size: c_int);
}

fn main() {
    let n = 5;
    let mut a = vec![1, 2, 3, 4, 5];
    let mut b = vec![10, 20, 30, 40, 50];
    let mut c = vec![0; n];

    // 🚀 FFI calls must be inside an `unsafe` block
    unsafe {
        run_cuda(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), n as c_int);
        print_array(c.as_mut_ptr(), n as c_int);
    }
}
