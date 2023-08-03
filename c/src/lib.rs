
#[no_mangle]
pub extern "C" fn temp() -> i32 {
    4
}

#[repr(C)]
pub struct MyStruct {
    pub a: i32
}

#[repr(C)]
pub struct MyResult {
    pub a: i32
}

impl MyStruct {
    #[no_mangle]
    pub extern "C" fn some_method(&self) -> MyResult {
        MyResult {
            a: self.a
        }
    }
}