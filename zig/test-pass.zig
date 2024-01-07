const std = @import("std");

const expect = std.testing.expect;

test "always pass" {
    try expect(true);
}

test "always fails" {
    try expect(false);
}
