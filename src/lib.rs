use std::ptr;

pub fn unsafe_string_copy(string: &String)-> String {
    unsafe {
        from_buf_raw(string.as_ptr(), string.len())
    }
}

unsafe fn from_buf_raw(ptr: *const u8, elts: usize) -> String {
    let mut dst = String::with_capacity(elts);

    ptr::copy(ptr, dst.as_mut_ptr(), elts);

    dst.as_mut_vec().set_len(elts);
    dst
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string = "agsdkagsdiasdiug8i3diw2insu2iuwnuisnin2".to_string();
        let result = unsafe_string_copy(&string);
        assert_eq!(string, result);
    }
}
