const expect = @import("std").testing.expect;

test "IF Expression" {
    var current: i32 = 0;
    if (current < 10) {
        current = 10;
    } else {
        current = 20;
    }
    try expect(current == 10);

    const any = if (current == 10) -10 else current;
    try expect(any == -10);
}
