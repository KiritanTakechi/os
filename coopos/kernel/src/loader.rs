extern "C" {
    static _num_app: usize;
}

fn get_app_num() -> usize {
    unsafe { _num_app }
}
