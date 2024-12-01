const std = @import("std");
const Allocator = std.mem.Allocator;
const max_bytes: usize = std.math.maxInt(usize);

pub fn read_file(path: []const u8, alloc: Allocator) ![]u8 {
    return try std.fs.cwd().readFileAlloc(alloc, path, max_bytes);
}
