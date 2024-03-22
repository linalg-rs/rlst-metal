#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod prelude;

pub use prelude::*;

pub mod raw {

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let device = unsafe { MetalDevice::from_default() };
        let mut buffer = unsafe { device.new_buffer(4, 0) };
        let contents = unsafe { buffer.contents::<f32>() };
        contents[0] = 5.0;
        println!("{}", contents[0]);
        AutoReleasePool::execute(|| {
            println!("Name: {}", unsafe { device.name() });
        });
    }
}
