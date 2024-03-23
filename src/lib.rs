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

    use rlst::dense::{rlst_array_from_slice5, tools::PrettyPrint};

    extern crate blas_src;
    extern crate lapack_src;

    use super::*;

    #[test]
    fn test_device() {
        let device = unsafe { MetalDevice::from_default() };
        AutoReleasePool::execute(|| {
            println!("Name: {}", unsafe { device.name() });
        });
    }

    #[test]
    fn test_matrix_descriptor() {
        AutoReleasePool::execute(|| {
            let rows = 5;
            let columns = 6;
            let matrices = 2;
            let row_bytes = columns * std::mem::size_of::<f32>();
            let matrix_bytes = columns * rows * std::mem::size_of::<f32>();
            let data_type = MpsDataType::F32;
            let matrix_desc = MpsMatrixDescriptor::new(
                rows,
                columns,
                matrices,
                row_bytes,
                matrix_bytes,
                data_type,
            );

            assert_eq!(unsafe { matrix_desc.rows() }, 5);
            assert_eq!(unsafe { matrix_desc.columns() }, 6);
            assert_eq!(unsafe { matrix_desc.row_bytes() }, row_bytes);
            assert_eq!(unsafe { matrix_desc.matrix_bytes() }, matrix_bytes);
            assert_eq!(unsafe { matrix_desc.matrices() }, 2);
        });
    }

    #[test]
    fn test_matrix() {
        AutoReleasePool::execute(|| {
            let rows = 5;
            let columns = 6;
            let matrices = 1;
            let row_bytes = columns * std::mem::size_of::<f32>();
            let matrix_bytes = columns * rows * std::mem::size_of::<f32>();
            let data_type = MpsDataType::F32;
            let matrix_desc = MpsMatrixDescriptor::new(
                rows,
                columns,
                matrices,
                row_bytes,
                matrix_bytes,
                data_type,
            );

            let nbytes = matrices * std::mem::size_of::<f32>() * rows * columns;

            let device = unsafe { MetalDevice::from_default() };

            let buffer = unsafe { MetalBuffer::new(&device, nbytes, 0) };

            let mps_matrix = unsafe { MpsMatrix::new(buffer, matrix_desc) };

            println!("{}", unsafe { mps_matrix.descriptor().rows() });
        });
    }

    #[test]
    fn test_matmul() {
        use rlst::prelude::*;
        AutoReleasePool::execute(|| {
            let rows = 2;
            let columns = 2;
            let matrices = 2;
            let row_bytes = columns * std::mem::size_of::<f32>();
            let matrix_bytes = columns * rows * std::mem::size_of::<f32>();
            let data_type = MpsDataType::F32;
            let matrix_left_desc = MpsMatrixDescriptor::new(
                rows,
                columns,
                matrices,
                row_bytes,
                matrix_bytes,
                data_type,
            );

            let matrix_right_desc = MpsMatrixDescriptor::new(
                rows,
                columns,
                matrices,
                row_bytes,
                matrix_bytes,
                data_type,
            );

            let matrix_result_desc = MpsMatrixDescriptor::new(
                rows,
                columns,
                matrices,
                row_bytes,
                matrix_bytes,
                data_type,
            );

            let nbytes = matrices * std::mem::size_of::<f32>() * rows * columns;

            let device = unsafe { MetalDevice::from_default() };

            let command_queue = unsafe { device.command_queue() };
            let mut command_buffer = unsafe { command_queue.command_buffer() };

            let buffer_left = unsafe { MetalBuffer::new(&device, nbytes, 0) };
            let buffer_right = unsafe { MetalBuffer::new(&device, nbytes, 0) };
            let buffer_result = unsafe { MetalBuffer::new(&device, nbytes, 0) };

            let mut mps_matrix_left = unsafe { MpsMatrix::new(buffer_left, matrix_left_desc) };
            let mut mps_matrix_right = unsafe { MpsMatrix::new(buffer_right, matrix_right_desc) };
            let mut mps_matrix_result =
                unsafe { MpsMatrix::new(buffer_result, matrix_result_desc) };

            let mut mat_left = rlst_array_from_slice_mut2!(
                f32,
                unsafe { mps_matrix_left.contents_mut::<f32>() },
                [2, 4]
            );

            mat_left.fill_from_seed_equally_distributed(0);

            let mut mat_right = rlst_array_from_slice_mut2!(
                f32,
                unsafe { mps_matrix_right.contents_mut::<f32>() },
                [2, 4]
            );

            mat_right.fill_from_seed_equally_distributed(0);

            let mut mat_result = rlst_array_from_slice_mut2!(
                f32,
                unsafe { mps_matrix_result.contents_mut::<f32>() },
                [2, 4]
            );

            mat_result.fill_from_seed_equally_distributed(1);

            let matmul =
                unsafe { MpsMatrixMultiplication::new(&device, true, true, 2, 2, 2, 1.0, 0.0) };

            unsafe {
                matmul.encode_to_command_buffer(
                    &mut command_buffer,
                    &mps_matrix_left,
                    &mps_matrix_right,
                    &mut mps_matrix_result,
                );
            }
            unsafe {
                command_buffer.commit();
                command_buffer.wait_until_completed();
            }

            let mat_left = rlst_array_from_slice2!(
                f32,
                unsafe { &mps_matrix_left.contents::<f32>()[4..8] },
                [2, 2]
            );

            let mat_right = rlst_array_from_slice2!(
                f32,
                unsafe { &mps_matrix_right.contents::<f32>()[4..8] },
                [2, 2]
            );

            let mat_result = rlst_array_from_slice2!(
                f32,
                unsafe { &mps_matrix_result.contents::<f32>()[4..8] },
                [2, 2],
                [2, 1]
            );

            let mut mat_expected = rlst_dynamic_array2!(f32, [2; 2]);

            mat_expected
                .view_mut()
                .simple_mult_into(mat_left.view(), mat_right.view());

            mat_expected.pretty_print();
            mat_result.pretty_print();
        });
    }
}
