
#include "rlst_metal.h"

rlst_mtl_autorelease_pool_p rlst_mtl_new_autorelease_pool() {
  return (rlst_mtl_autorelease_pool_p)[[NSAutoreleasePool alloc] init];
}

void rlst_mtl_autorelease_pool_drain(rlst_mtl_autorelease_pool_p p_pool) {
  [(NSAutoreleasePool *)p_pool drain];
}

rlst_mtl_device_p rlst_mtl_new_default_device() {
  id<MTLDevice> device = MTLCreateSystemDefaultDevice();
  return (rlst_mtl_device_p)device;
}

char *rlst_mtl_device_name(rlst_mtl_device_p p_device) {
  id<MTLDevice> device = (id<MTLDevice>)p_device;
  return (char *)[device.name UTF8String];
};

rlst_mtl_buffer_p rlst_mtl_device_new_buffer(rlst_mtl_device_p p_device,
                                             unsigned long length,
                                             unsigned int options) {
  id<MTLDevice> device = (id<MTLDevice>)p_device;
  return (rlst_mtl_buffer_p)[device newBufferWithLength:length options:options];
}

rlst_mtl_command_queue_p rlst_mtl_device_new_command_queue(rlst_mtl_device_p p_device) {
  return (rlst_mtl_command_queue_p)[(id<MTLDevice>)p_device newCommandQueue];
}

void rlst_mtl_buffer_release(rlst_mtl_buffer_p p_buffer) {

  id<MTLBuffer> buffer = (id<MTLBuffer>)p_buffer;
  return [buffer release];
}

void *rlst_mtl_buffer_contents(rlst_mtl_buffer_p p_buffer) {
  id<MTLBuffer> buffer = (id<MTLBuffer>)buffer;
  return [buffer contents];
}

/* Command Queue */

void rlst_mtl_command_queue_release(rlst_mtl_command_queue_p p_queue) {
  [id<MTLCommandQueue>p_queue release];
}

rlst_mtl_command_buffer_p rlst_mtl_command_queue_command_buffer(rlst_mtl_command_queue_p p_queue) {
  return (rlst_mtl_command_buffer_p)[id<MTLCommandQueue>p_queue commandBuffer];
}

/* Command Buffer */

void rlst_mtl_command_buffer_wait_until_completed(rlst_mtl_command_buffer_p p_buffer) {
  [(id<MTLCommandBuffer>)p_buffer waitUntilCompleted];
}

void rlst_mtl_command_buffer_commit(rlst_mtl_command_buffer_p p_buffer) {
    [(id<MTLCommandBuffer>)p_buffer commit];
}

rlst_mtl_compute_command_encoder_p rlst_mtl_command_buffer_compute_command_encoder(rlst_mtl_command_buffer_p p_buffer, unsigned int dispatch_type){
  return (rlst_mtl_compute_command_encoder_p)[(id<MTLCommandBuffer>)p_buffer computeCommandEncoderWithDispatchType: dispatch_type]
}
