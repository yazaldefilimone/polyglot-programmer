const std = @import("std");

pub fn main() void {
    const constant: i32 = 10;
    _ = constant;
    var variable: u32 = 500;
    _ = variable;

    const inferrend_constant = @as(i32, 5);
    _ = inferrend_constant;
    var inferrend_variable = @as(u32, 500);
    _ = inferrend_variable;

    const a: i32 = undefined;
    _ = a;
    const b: u32 = undefined;
    _ = b;
}
