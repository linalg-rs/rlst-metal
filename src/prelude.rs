//! Interface routines

use crate::raw;

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

    pub unsafe fn contents<T: Sized>(&mut self) -> &mut [T] {
        let ptr = raw::rlst_mtl_buffer_contents(self.buffer_p);
        std::slice::from_raw_parts_mut(ptr as *mut T, self.nbytes / std::mem::size_of::<T>())
    }
}

impl Drop for MetalBuffer {
    fn drop(&mut self) {
        unsafe {
            raw::rlst_mtl_buffer_release(self.buffer_p);
        }
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
