use std::fmt;
use std::iter::IntoIterator;
use std::iter::Iterator;

fn main() {
    //test1();
    //test2();
    //test3();
    //test4();
    //test5();
    test6();
}

fn test1() {
    println!("Start!");
    let mut b = BTree::<i64, i64>::new();
    b.insert(10, 10);
    b.insert(20, 20);
    b.print();

    b.insert(30, 30);
    b.print();

    b.insert(25, 25);
    b.print();

    b.insert(15, 15);
    b.print();

    b.insert(0, 0);
    b.print();

    b.insert(35, 35);
    b.print();

    println!("20: {:?}", b.find(&20));
    println!("100: {:?}", b.find(&100));
    println!("0: {:?}", b.find(&0));
    println!("1: {:?}", b.find(&1));

    println!("Done");
}

fn test2() {
    println!("Start!");
    let mut b = BTree::<i64, i64>::new_with(3);
    b.insert(10, 10);
    b.insert(20, 20);

    b.insert(30, 30);

    b.insert(25, 25);

    b.insert(15, 15);

    b.insert(0, 0);

    b.insert(35, 35);

    b.print();
    println!("Iterator!");
    for x in b.iter() {
        println!("{:?}", x)
    }

    println!("Done");
}

fn test3() {
    println!("Start!");
    let mut b = BTree::<i64, i64>::new_with(5);
    b.insert(10, 10);
    b.insert(20, 20);

    b.insert(30, 30);

    b.insert(25, 25);

    b.insert(15, 15);

    b.insert(0, 0);

    b.insert(35, 35);

    b.print();
    println!("\nFrom 10");
    let mut it = b.iter();
    it.move_to(&10);
    for x in it {
        println!("{:?}", x)
    }

    println!("\nFrom 30");
    it = b.iter();
    it.move_to(&30);
    for x in it {
        println!("{:?}", x)
    }

    println!("\nFrom 35");
    it = b.iter();
    it.move_to(&35);
    for x in it {
        println!("{:?}", x)
    }

    println!("\nFrom -1");
    it = b.iter();
    it.move_to(&-1);
    for x in it {
        println!("{:?}", x)
    }

    println!("\nFrom 13");
    it = b.iter();
    it.move_to(&13);
    for x in it {
        println!("{:?}", x)
    }

    println!("\nFrom 100");
    it = b.iter();
    it.move_to(&100);
    for x in it {
        println!("{:?}", x)
    }

    println!("Done");
}

fn test4() {
    println!("Test4 m=5");
    let mut b = BTree::<i64, i64>::new_with(5);
    b.insert(10, 10);
    b.insert(20, 20);
    b.print();

    b.insert(30, 30);
    b.print();

    b.insert(25, 25);
    b.print();

    b.insert(15, 15);
    b.print();

    b.insert(0, 0);
    b.print();

    b.insert(35, 35);
    b.print();

    b.insert(40, 40);
    b.print();

    b.insert(-20, -20);
    b.print();

    b.insert(-10, -10);
    b.print();

    println!("20: {:?}", b.find(&20));
    println!("100: {:?}", b.find(&100));
    println!("0: {:?}", b.find(&0));
    println!("1: {:?}", b.find(&1));

    println!("Done");
}

fn test5() {
    println!("Test5 other features");
    let mut b = BTree::<i64, i64>::new_with(5);
    println!("{}", b.is_empty());
    b.insert(10, 10);
    println!("{}", b.is_empty());
}

fn test6() {
    println!("Test6 Remove");
    let mut b = BTree::<i64, i64>::new_with(5);
    b.insert(10, 10);
    b.insert(20, 20);
    b.insert(30, 30);
    b.insert(40, 40);
    
    b.print();

    let x = b.remove(&20);
    println!("20: {:?}", x);
    b.print();
}

struct BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    m: usize,
    root: Node<K, V>,
}

// Separate keys and values.
// In the search, I need to acess only keys.
// Having keys as a dedicated data structure makes
// the search faster.
// More work is required in the insert, but it does not matter
// in a usual case.
struct Node<K, V>
where
    K: Ord,
{
    ks: Vec<K>,
    vs: Vec<V>,
    ns: Vec<Node<K, V>>,
}

struct NodeIter<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    node: &'a Node<K, V>,
    pos: usize,
    go_child: bool,
}

struct BTreeIterator<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    stack: Vec<NodeIter<'a, K, V>>,
    curr: NodeIter<'a, K, V>,
}

impl<K, V> BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn new() -> BTree<K, V> {
        BTree::<K, V>::new_with(3)
    }

    fn new_with(order: usize) -> BTree<K, V> {
        BTree::<K, V> {
            m: order,
            root: Node::<K, V>::new(order),
        }
    }

    fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    fn iter_from(&self, x: &K) -> BTreeIterator<K, V> {
        let mut it = self.iter();

        loop {
            let pos = it.curr.node.find_pos(x);
            if pos.1 {
                it.curr.pos = pos.0;
                it.curr.go_child = false;
            }

            if it.curr.node.is_leaf() {}
        }
    }

    fn iter(&self) -> BTreeIterator<K, V> {
        BTreeIterator {
            stack: Vec::new(),
            curr: NodeIter {
                node: &self.root,
                pos: 0,
                go_child: true,
            },
        }
    }

    fn insert(&mut self, x: K, v: V) {
        let need_split = self.root.insert(x, v, self.m);
        if need_split {
            let mut tmp = Node::new(self.m);
            std::mem::swap(&mut tmp, &mut self.root);

            let split = tmp.split(self.m);

            self.root.ks.push(split.0);
            self.root.vs.push(split.1);
            self.root.ns.push(tmp);
            self.root.ns.push(split.2);
        }
    }

    fn remove(&mut self, x: &K) -> Option<V> {
        let v = self.root.remove(x);

        return v;
    }

    fn find(&self, x: &K) -> Option<&V> {
        return self.root.find(x);
    }

    fn print(&self) {
        println!("{}", self.root);

        let mut next = Vec::<&Node<K, V>>::with_capacity(10);
        for n in &self.root.ns {
            next.push(n);
        }

        loop {
            if next.len() == 0 {
                break;
            }

            for n in &next {
                print!("{} ", n);
            }
            println!();

            let mut tmp = Vec::<&Node<K, V>>::with_capacity(10);
            for n in &next {
                for c in &n.ns {
                    tmp.push(c);
                }
            }

            std::mem::swap(&mut tmp, &mut next);
        }

        println!();
    }
}

impl<'a, K, V> IntoIterator for &'a BTree<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    type Item = (&'a K, &'a V);
    type IntoIter = BTreeIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> Node<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn new(m: usize) -> Node<K, V> {
        Node::<K, V> {
            ks: Vec::<K>::with_capacity(m - 1),
            vs: Vec::<V>::with_capacity(m - 1),
            ns: Vec::<Node<K, V>>::with_capacity(m),
        }
    }

    fn is_leaf(&self) -> bool {
        self.ns.is_empty()
    }

    fn is_empty(&self) -> bool {
        self.ks.is_empty()
    }

    

    fn insert(&mut self, x: K, v: V, m: usize) -> bool {
        let pos = self.find_pos(&x);
        if self.is_leaf() {
            if pos.1 {
                // overwrite
                self.ks[pos.0] = x;
                self.vs[pos.0] = v;
                return false;
            }
            // insert
            self.ks.insert(pos.0, x);
            self.vs.insert(pos.0, v);
            return self.ks.len() == m;
        }

        let need_split = self.ns[pos.0].insert(x, v, m);
        if need_split {
            self.split_child(pos.0, m);
            return self.ks.len() == m;
        }

        return false;
    }

    fn remove(&mut self, x: &K, m:usize) -> Option<V> {
        let pos = self.find_pos(&x);
        if self.is_leaf() {
            // 1. Leaf
            if !pos.1 {
                // Not found
                return None;
            }
            self.ks.remove(pos.0);
            return Some(self.vs.remove(pos.0));
        }

        // 2. Node
        if pos.1 {
            // 2.1. Remove from this node

        }
        
        // 2.2. Remove from child


        
        return None;
    }

    fn find(&self, x: &K) -> Option<&V> {
        let pos = self.find_pos(x);
        if pos.1 {
            return Some(&self.vs[pos.0]);
        }

        if self.ns.len() == 0 {
            return None;
        }

        return self.ns[pos.0].find(x);
    }

    fn split_child(&mut self, pos: usize, m: usize) {
        let result = self.ns[pos].split(m);
        self.ks.insert(pos, result.0);
        self.vs.insert(pos, result.1);
        self.ns.insert(pos + 1, result.2);
    }

    // Split self into two nodes (self and right).
    fn split(&mut self, m: usize) -> (K, V, Node<K, V>) {
        let mut right = Node::new(m);

        let mid = m / 2;

        right.ks = self.ks.split_off(mid + 1);
        right.vs = self.vs.split_off(mid + 1);

        let midE = self.ks.pop().unwrap();
        let midV = self.vs.pop().unwrap();

        if self.ns.len() > 0 {
            right.ns = self.ns.split_off(mid + 1);
        }
        return (midE, midV, right);
    }

    fn find_pos(&self, x: &K) -> (usize, bool) {
        for i in 0..self.ks.len() {
            if x < &self.ks[i] {
                return (i, false);
            } else if x == &self.ks[i] {
                return (i, true);
            }
        }
        return (self.ks.len(), false);
    }

    fn remove_right_most(&mut self, m: usize) -> (K, V, bool) {
        if self.is_leaf() {
            // 1. Leaf
            let last = self.ks.len() - 1;
            let k = self.ks.remove(last);
            let v = self.vs.remove(last);
            return (k, v, self.is_empty());
        }
        // 2. Node (remove from child)
        let last = self.ns.len() - 1;
        let child = self.ns.get(last).unwrap();
        let result = child.remove_right_most(m);
        if result.2 {
            borrow_or_merge_from_l(last);
        }
        return (result.0, result.1, self.ks.len() < m/2 )
    }

}

impl<K, V> fmt::Display for Node<K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        if self.ks.len() > 0 {
            write!(f, "{}", self.ks[0]);
            for i in 1..self.ks.len() {
                write!(f, ",{}", self.ks[i]);
            }
        }
        write!(f, "]");
        Ok(())
    }
}

impl<'a, K, V> BTreeIterator<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    fn move_to(&mut self, x: &K) {
        loop {
            let pos = self.curr.node.find_pos(x);
            self.curr.pos = pos.0;
            if pos.1 {
                self.curr.go_child = false;
                return;
            }

            if self.curr.node.is_leaf() {
                return;
            }

            self.curr.go_child = false;
            let child = &self.curr.node.ns[pos.0];
            let mut tmp = NodeIter {
                node: child,
                pos: 0,
                go_child: false,
            };
            std::mem::swap(&mut tmp, &mut self.curr);
            self.stack.push(tmp);
        }
    }
}

impl<'a, K, V> Iterator for BTreeIterator<'a, K, V>
where
    K: Ord,
    K: fmt::Display,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr.node.is_leaf() {
                if self.curr.pos < self.curr.node.ks.len() {
                    // Get the right neighbor
                    let key = &self.curr.node.ks[self.curr.pos];
                    let val = &self.curr.node.vs[self.curr.pos];
                    self.curr.pos += 1;
                    return Some((key, val));
                } else {
                    match self.stack.pop() {
                        Some(x) => {
                            self.curr = x;
                            continue;
                        }
                        None => {
                            return None;
                        }
                    }
                }
            } else {
                if self.curr.go_child {
                    loop {
                        self.curr.go_child = false;
                        let child = &self.curr.node.ns[self.curr.pos];
                        let mut tmp = NodeIter {
                            node: child,
                            pos: 0,
                            go_child: false,
                        };

                        std::mem::swap(&mut tmp, &mut self.curr);
                        self.stack.push(tmp);

                        if child.is_leaf() {
                            break;
                        }
                    }
                    continue;
                } else {
                    if self.curr.pos < self.curr.node.ks.len() {
                        // Get the right neighbor
                        let key = &self.curr.node.ks[self.curr.pos];
                        let val = &self.curr.node.vs[self.curr.pos];
                        self.curr.pos += 1;
                        self.curr.go_child = true;
                        return Some((key, val));
                    } else {
                        match self.stack.pop() {
                            Some(x) => {
                                self.curr = x;
                                continue;
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }
}
