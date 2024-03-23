//! Interface routines

use crate::raw;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ResourceOptions {
    StorageModeManaged = raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_STORAGE_MODE_MANAGED,
    StorageModePrivate = raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_STORAGE_MODE_PRIVATE,
    StorageModeMemoryless = raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_STORAGE_MODE_MEMORYLESS,
    HazardTrackingModeUntracked =
        raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED,
    HazardTrackingModeTracked =
        raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_HAZARD_TRACKING_MODE_TRACKED,
    CpuCacheModeWriteCombined =
        raw::RLSTMtlResourceOptions_RLST_MTL_RESOURCE_CPU_CACHE_MODE_WRITE_COMBINED,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum MpsDataType {
    F32 = raw::RLSTMtlMpsDataType_RLST_MTL_MPS_FLOAT32,
}

pub struct AutoReleasePool;

impl AutoReleasePool {
    pub fn execute(fun: impl Fn()) {
        let pool = unsafe { raw::rlst_mtl_new_autorelease_pool() };
        fun();
        unsafe { raw::rlst_mtl_autorelease_pool_drain(pool) };
    }
}

pub struct MetalDevice {
    device_p: raw::rlst_mtl_device_p,
}

impl std::ops::Drop for MetalDevice {
    fn drop(&mut self) {
        unsafe { raw::rlst_mtl_device_release(self.device_p) }
    }
}

impl MetalDevice {
    pub unsafe fn from_default() -> Self {
        Self {
            device_p: raw::rlst_mtl_new_default_device(),
        }
    }
    pub unsafe fn new_buffer(&self, nbytes: usize, options: u32) -> MetalBuffer {
        MetalBuffer::new(self, nbytes, options)
    }

    pub unsafe fn name(&self) -> String {
        std::ffi::CStr::from_ptr(raw::rlst_mtl_device_name(self.device_p))
            .to_str()
            .map(|s| s.to_owned())
            .unwrap()
    }

    pub unsafe fn command_queue(&self) -> MetalCommandQueue {
        MetalCommandQueue {
            queue_p: raw::rlst_mtl_device_new_command_queue(self.device_p),
        }
    }
}

pub struct MetalCommandQueue {
    queue_p: raw::rlst_mtl_command_queue_p,
}

impl MetalCommandQueue {
    pub unsafe fn command_buffer(&self) -> MetalCommandBuffer {
        MetalCommandBuffer {
            command_buffer_p: raw::rlst_mtl_command_queue_command_buffer(self.queue_p),
        }
    }
}

impl Drop for MetalCommandQueue {
    fn drop(&mut self) {
        unsafe {
            raw::rlst_mtl_command_queue_release(self.queue_p);
        }
    }
}

pub struct MetalBuffer {
    buffer_p: raw::rlst_mtl_buffer_p,
    nbytes: usize,
}

impl MetalBuffer {
    pub unsafe fn new(device: &MetalDevice, nbytes: usize, options: u32) -> MetalBuffer {
        Self {
            buffer_p: raw::rlst_mtl_device_new_buffer(device.device_p, nbytes as u64, options),
            nbytes,
        }
    }

    pub unsafe fn contents_mut<T: Sized>(&mut self) -> &mut [T] {
        let ptr = raw::rlst_mtl_buffer_contents(self.buffer_p);
        std::slice::from_raw_parts_mut(ptr as *mut T, self.nbytes / std::mem::size_of::<T>())
    }

    pub unsafe fn contents<T: Sized>(&self) -> &[T] {
        let ptr = raw::rlst_mtl_buffer_contents(self.buffer_p);
        std::slice::from_raw_parts(ptr as *mut T, self.nbytes / std::mem::size_of::<T>())
    }
}

impl Drop for MetalBuffer {
    fn drop(&mut self) {
        unsafe {
            raw::rlst_mtl_buffer_release(self.buffer_p);
        }
    }
}

pub struct MetalCommandBuffer {
    command_buffer_p: raw::rlst_mtl_command_buffer_p,
}

impl MetalCommandBuffer {
    pub unsafe fn commit(&self) {
        raw::rlst_mtl_command_buffer_commit(self.command_buffer_p);
    }

    pub unsafe fn wait_until_completed(&self) {
        raw::rlst_mtl_command_buffer_wait_until_completed(self.command_buffer_p);
    }
}

pub struct MpsMatrixDescriptor {
    desc: raw::rlst_mtl_mps_matrix_descriptor_p,
}

impl MpsMatrixDescriptor {
    pub fn new(
        rows: usize,
        columns: usize,
        matrices: usize,
        row_bytes: usize,
        matrix_bytes: usize,
        data_type: MpsDataType,
    ) -> Self {
        Self {
            desc: unsafe {
                raw::rlst_mtl_mps_matrix_descriptor(
                    rows as u64,
                    columns as u64,
                    matrices as u64,
                    row_bytes as u64,
                    matrix_bytes as u64,
                    data_type as u32,
                )
            },
        }
    }

    pub unsafe fn row_bytes_from_columns(columns: usize, data_type: MpsDataType) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_row_bytes_from_columns(columns as u64, data_type as u32)
    }

    pub unsafe fn rows(&self) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_rows(self.desc) as usize
    }

    pub unsafe fn columns(&self) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_columns(self.desc) as usize
    }

    pub unsafe fn matrices(&self) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_matrices(self.desc) as usize
    }

    pub unsafe fn row_bytes(&self) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_row_bytes(self.desc) as usize
    }

    pub unsafe fn matrix_bytes(&self) -> usize {
        raw::rlst_mtl_mps_matrix_descriptor_matrix_bytes(self.desc) as usize
    }
}

pub struct MpsMatrix {
    matrix_p: raw::rlst_mtl_mps_matrix_p,
    buffer: MetalBuffer,
    descriptor: MpsMatrixDescriptor,
}

impl MpsMatrix {
    pub unsafe fn new(buffer: MetalBuffer, descriptor: MpsMatrixDescriptor) -> Self {
        Self {
            matrix_p: raw::rlst_mtl_mps_matrix(buffer.buffer_p, descriptor.desc),
            buffer,
            descriptor,
        }
    }

    pub unsafe fn buffer(&self) -> &MetalBuffer {
        &self.buffer
    }

    pub unsafe fn descriptor(&self) -> &MpsMatrixDescriptor {
        &self.descriptor
    }

    pub unsafe fn contents_mut<T: Sized>(&mut self) -> &mut [T] {
        self.buffer.contents_mut()
    }

    pub unsafe fn contents<T: Sized>(&self) -> &[T] {
        self.buffer.contents()
    }
}

impl Drop for MpsMatrix {
    fn drop(&mut self) {
        unsafe {
            raw::rlst_mtl_mps_matrix_release(self.matrix_p);
        }
    }
}

pub struct MpsMatrixMultiplication {
    matrix_mult_p: raw::rlst_mtl_mps_matrix_multiplication_p,
}

impl MpsMatrixMultiplication {
    pub unsafe fn new(
        device: &MetalDevice,
        transpose_left: bool,
        transpose_right: bool,
        result_rows: usize,
        result_columns: usize,
        interior_columns: usize,
        alpha: f64,
        beta: f64,
    ) -> Self {
        Self {
            matrix_mult_p: raw::rlst_mtl_mps_matrix_multiplication(
                device.device_p,
                transpose_left,
                transpose_right,
                result_rows as u64,
                result_columns as u64,
                interior_columns as u64,
                alpha,
                beta,
            ),
        }
    }

    pub unsafe fn encode_to_command_buffer(
        &self,
        command_buffer: &mut MetalCommandBuffer,
        left_matrix: &MpsMatrix,
        right_matrix: &MpsMatrix,
        result_matrix: &mut MpsMatrix,
    ) {
        raw::rlst_mtl_mps_matrix_multiplication_encode_to_command_buffer(
            self.matrix_mult_p,
            command_buffer.command_buffer_p,
            left_matrix.matrix_p,
            right_matrix.matrix_p,
            result_matrix.matrix_p,
        );
    }
}

impl Drop for MpsMatrixMultiplication {
    fn drop(&mut self) {
        unsafe {
            raw::rlst_mtl_mps_matrix_multiplication_release(self.matrix_mult_p);
        }
    }
}
