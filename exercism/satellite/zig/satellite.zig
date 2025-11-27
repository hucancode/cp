const std = @import("std");
const mem = std.mem;

pub const TraversalError = error{
    DifferentLengths,
    DifferentItems,
    NonUniqueItems,
};

pub const Node = struct {
    data: u8,
    left: ?*Node,
    right: ?*Node,
    pub fn initFromTraversals(self: *Node, allocator: mem.Allocator, preorder: []const u8, inorder: []const u8) (mem.Allocator.Error || TraversalError)!void {
        self.left = null;
        self.right = null;
        const i = mem.indexOfScalar(u8, inorder, preorder[0]);
        if (i == null) return TraversalError.DifferentItems;
        self.data = preorder[0];
        errdefer self.deinit(allocator);
        const linorder = inorder[0..i.?];
        const lpreorder = preorder[1..linorder.len+1];
        const rinorder = inorder[i.?+1..inorder.len];
        const rpreorder = preorder[linorder.len+1..preorder.len];
        if (linorder.len > 0) {
            self.left = try allocator.create(Node);
            try self.left.?.initFromTraversals(allocator, lpreorder, linorder);
        }
        if (rinorder.len > 0) {
            self.right = try allocator.create(Node);
            try self.right.?.initFromTraversals(allocator, rpreorder, rinorder);
        }
    }

    pub fn deinit(self: *Node, allocator: mem.Allocator) void {
        if (self.left) |node| {
            node.deinit(allocator);
            allocator.destroy(node);
        }
        if (self.right) |node| {
            node.deinit(allocator);
            allocator.destroy(node);
        }
    }
};

pub const Tree = struct {
    root: ?*Node,
    allocator: mem.Allocator,
    
    pub fn initFromTraversals(allocator: mem.Allocator, preorder: []const u8, inorder: []const u8) (mem.Allocator.Error || TraversalError)!Tree {
        try validate(preorder, inorder);
        var tree = Tree {
            .allocator = allocator,
            .root = null,
        };
        if (preorder.len == 0) return tree;
        tree.root = try allocator.create(Node);
        errdefer tree.deinit();
        try tree.root.?.initFromTraversals(allocator, preorder, inorder);
        return tree;
    }

    fn validate(a: []const u8, b: []const u8) TraversalError!void {
        if (a.len != b.len) return TraversalError.DifferentLengths;
        var ahash = mem.zeroes([256]bool);
        var bhash = mem.zeroes([256]bool);
        for (a,b) |ca,cb| {
            if(ahash[ca] or bhash[cb]) {
                return TraversalError.NonUniqueItems;
            }
            ahash[ca] = true;
            bhash[cb] = true;
        }
        if (!mem.eql(bool, &ahash, &bhash)) {
            return TraversalError.DifferentItems;
        }
    }

    pub fn deinit(self: *Tree) void {
        if (self.root) |node|{
            node.deinit(self.allocator);
            self.allocator.destroy(node);
        }
    }
};
