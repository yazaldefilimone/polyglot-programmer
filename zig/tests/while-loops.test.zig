const expect = @import("std").testing.expect;
const print = @import("std").debug.print;

test "simple while loop" {
    var i: i16 = 0;
    while (i < 10) {
        i += 1;
    }
    try expect(i == 10);
}

test "while with cintinua expression" {
    var index: i32 = 0;
    var sum: i32 = 0;
    while (index < 10) : (index += 1) {
        sum += index;
    }
    try expect(sum == 45);
}

test "while with continue" {
    var sum: u8 = 0;
    var i: u8 = 0;
    while (i <= 3) : (i += 1) {
        if (i == 2) continue;
        sum += i;
    }
    try expect(sum == 4);
}

test "while with break" {
    var sum: u8 = 0;
    var i: u8 = 0;
    while (i <= 3) : (i += 1) {
        if (i == 2) break;
        sum += i;
    }
    try expect(sum == 1);
}
