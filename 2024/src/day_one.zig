const com = @import("common.zig");
const std = @import("std");
const ttg = std.testing;
const Allocator = std.mem.Allocator;

fn part_one(text: []const u8, allocator: Allocator) !i32 {
    var split = std.mem.splitScalar(u8, text, '\n');
    var total: i32 = 0;

    var left_arr = std.ArrayList(i32).init(allocator);
    defer left_arr.deinit();
    var right_arr = std.ArrayList(i32).init(allocator);
    defer right_arr.deinit();

    while (split.next()) |line| {
        if (std.mem.eql(u8, "", line)) {
            continue;
        }

        var line_split = std.mem.splitSequence(u8, line, "   ");
        const left = line_split.next().?;
        const right = line_split.next().?;
        const left_num = try std.fmt.parseInt(i32, left, 10);
        const right_num = try std.fmt.parseInt(i32, right, 10);
        try left_arr.append(left_num);
        try right_arr.append(right_num);
    }

    std.mem.sort(i32, left_arr.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right_arr.items, {}, comptime std.sort.asc(i32));

    for (left_arr.items, right_arr.items) |lhs, rhs| {
        total += @intCast(@abs(rhs - lhs));
    }

    return total;
}

fn part_two(text: []const u8, allocator: Allocator) !u32 {
    var split = std.mem.splitScalar(u8, text, '\n');
    var sim_score: u32 = 0;

    var left_arr = std.ArrayList(i32).init(allocator);
    defer left_arr.deinit();
    var right_arr = std.ArrayList(i32).init(allocator);
    defer right_arr.deinit();

    while (split.next()) |line| {
        if (std.mem.eql(u8, "", line)) {
            continue;
        }

        var line_split = std.mem.splitSequence(u8, line, "   ");
        const left = line_split.next().?;
        const right = line_split.next().?;
        const left_num = try std.fmt.parseInt(i32, left, 10);
        const right_num = try std.fmt.parseInt(i32, right, 10);
        try left_arr.append(left_num);
        try right_arr.append(right_num);
    }

    var tally = std.AutoHashMap(i32, u32).init(allocator);
    defer tally.deinit();

    for (right_arr.items) |i| {
        if (tally.contains(i)) {
            try tally.put(i, tally.get(i).? + 1);
            continue;
        }

        try tally.put(i, 1);
    }

    for (left_arr.items) |i| {
        if (tally.contains(i)) {
            const as_unsigned: u32 = @intCast(i);
            sim_score += as_unsigned * tally.get(i).?;
        }
    }

    return sim_score;
}

test "part one example" {
    const example =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    try ttg.expectEqual(11, try part_one(example, ttg.allocator));
}

test "part one with data" {
    var arena = std.heap.ArenaAllocator.init(ttg.allocator);
    defer arena.deinit();

    const data = try com.read_file("data/day_one.txt", arena.allocator());
    try ttg.expectEqual(2769675, try part_one(data, arena.allocator()));
}

test "part two example" {
    const example =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    try ttg.expectEqual(31, try part_two(example, ttg.allocator));
}

test "part two with data" {
    var arena = std.heap.ArenaAllocator.init(ttg.allocator);
    defer arena.deinit();

    const data = try com.read_file("data/day_one.txt", arena.allocator());
    try ttg.expectEqual(24643097, try part_two(data, arena.allocator()));
}
