const std = @import("std");
pub fn main() void {
    try call();
}

pub fn call() !void {
    var i: usize = 0;
    var numberBytes: [@sizeOf(u8)]u8 = undefined;
    while (i < 1e7) {
        std.crypto.random.bytes(numberBytes[0..]);
        i += 1;
    }
}
