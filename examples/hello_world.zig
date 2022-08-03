const std = @import("std");

extern fn args_get(argv_offset: i32, argv_buffer_offset: i32) i32;
extern fn random_get(buffer_offset: *[]u8, buffer_len: i32) i32;

export fn main() i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(!gpa.deinit());
    const allocator = gpa.allocator();

    var ptr = allocator.alloc(u8, 16) catch unreachable;
    var status = random_get(&ptr, 16);
    if (status != 0) {
        return status;
    }
    return 0;
}
