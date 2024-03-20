
#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
#import <MetalPerformanceShaders/MetalPerformanceShaders.h>

typedef struct p_mtl_device_s *p_mtl_device;
typedef struct p_autorelease_pool_s *p_autorelease_pool;
typedef struct p_mtl_buffer_s *p_mtl_buffer;

p_autorelease_pool new_autorelease_pool() {
  return (p_autorelease_pool)[[NSAutoreleasePool alloc] init];
}

enum ResourceOptions {
  MTL_RESOURCE_STORAGE_MODE_SHARED = (unsigned int)MTLResourceStorageModeShared,
  MTL_RESOURCE_STORAGE_MODE_MANAGED =
      (unsigned int)MTLResourceStorageModeManaged,
  MTL_RESOURCE_STORAGE_MODE_PRIVATE =
      (unsigned int)MTLResourceStorageModePrivate,
  MTL_RESOURCE_STORAGE_MODE_MEMORYLESS =
      (unsigned int)MTLResourceStorageModeMemoryless,
  MTL_RESOURCE_HAZARD_TRACKING_MODE_DEFAULT =
      (unsigned int)MTLResourceHazardTrackingModeDefault,
  MTL_RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED =
      (unsigned int)MTLResourceHazardTrackingModeUntracked,
  MTL_RESOURCE_HAZARD_TRACKING_MODE_TRACKED =
      (unsigned int)MTLResourceHazardTrackingModeTracked,
  MTL_RESOURCE_CPU_CACHE_MODE_WRITE_COMBINED =
      (unsigned int)MTLResourceCPUCacheModeWriteCombined,
  MTL_RESOURCE_CPU_CACHE_MODE_DEFAULT =
      (unsigned int)MTLResourceCPUCacheModeDefaultCache,

};

void autorelease_pool_drain(p_autorelease_pool pool) {
  [(NSAutoreleasePool *)pool drain];
}

p_mtl_device new_default_device() {
  id<MTLDevice> device = MTLCreateSystemDefaultDevice();
  return (p_mtl_device)device;
}

char *device_get_name(p_mtl_device p_device) {
  id<MTLDevice> device = (id<MTLDevice>)p_device;
  return (char *)[device.name UTF8String];
}

p_mtl_buffer device_new_buffer(p_mtl_device p_device, int length,
                               int mtl_storage_options) {
  id<MTLDevice> device = (id<MTLDevice>)p_device;
  id<MTLBuffer> buffer = [device newBufferWithLength:length
                                             options:mtl_storage_options];
  return (p_mtl_buffer)buffer;
}

void *buffer_contents(p_mtl_buffer p_buffer) {
  id<MTLBuffer> buffer = (id<MTLBuffer>)buffer;
  return [buffer contents];
}

void buffer_release(p_mtl_buffer p_buffer) {
  id<MTLBuffer> buffer = (id<MTLBuffer>)p_buffer;
  [buffer release];
}

void device_release(p_mtl_device p_device) {
  id<MTLDevice> device = (id<MTLDevice>)p_device;

  [device release];
}
