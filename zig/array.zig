const std = @import("std");

pub fn main() void {
    var array = [2]u32{ 1, 2 };

    const len = array.len;
    const result: bool = len == 2;
    std.debug.print("{}\n", .{result});

    const newarray = [_]u8{ 'h', 'e', 'l', 'l', 'o' };
    const length = newarray.len;
    std.debug.print("{} and {}", .{ length, newarray });
}
