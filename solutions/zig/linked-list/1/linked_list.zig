pub fn LinkedList(comptime T: type) type {
    return struct {
        pub const Self = @This();
        pub const Node = struct {
            prev: ?*Node = null,
            next: ?*Node = null,
            data: T,
        };

        first: ?*Node = null,
        last: ?*Node = null,
        len: usize = 0,

        pub fn push(self: *Self, node: *Node) void {
            if (self.len == 0) {
                self.first = node;
                self.last = node;
            } else {
                node.prev = self.last;
                self.last.?.next = node;
                self.last = node;
            }
            self.len += 1;
        }

        pub fn pop(self: *Self) ?*Node {
            const ret = self.last;
            if (self.last != null) {
                self.last = self.last.?.prev;
                self.len -= 1;
            }
            if (self.len == 0) {
                self.first = null;
            }
            return ret;
        }

        pub fn shift(self: *Self) ?*Node {
            const ret = self.first;
            if (self.first != null) {
                self.first = self.first.?.next;
                self.len -= 1;
            }
            if (self.len == 0) {
                self.last = null;
            }
            return ret;
        }

        pub fn unshift(self: *Self, node: *Node) void {
            if (self.len == 0) {
                self.first = node;
                self.last = node;
            } else {
                self.first.?.prev = node;
                node.next = self.first;
                self.first = node;
            }
            self.len += 1;
        }

        fn contains(self: *Self, node: *Node) bool {
            var p = self.first;
            while (p != null) {
                if (node == p) {
                    return true;
                }
                p = p.?.next;
            }
            return false;
        }

        pub fn delete(self: *Self, node: *Node) void {
            if (!self.contains(node)) {
                return;
            }
            if (node != self.first and node != self.last) {
                const prev = node.prev;
                const next = node.next;
                prev.?.next = next;
                next.?.prev = prev;
            }
            if (node == self.first) {
                self.first = node.next;
            }
            if (node == self.last) {
                self.last = node.prev;
            }
            self.len -= 1;
        }
    };
}
