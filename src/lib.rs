#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//const out_dir: &'static str = env!("OUT_DIR");

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        unsafe {
            let device = rlst_mtl_new_default_device();
            let name = std::ffi::CStr::from_ptr(rlst_mtl_device_name(device))
                .to_str()
                .map(|s| s.to_owned())
                .unwrap();
            println!("{}", name);
        }
    }
}
