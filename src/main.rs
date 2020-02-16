use std::fmt;

fn main() {
    println!("Start!");
    let mut b = BTree::<i64, i64>::new(3);
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

struct BTree<K, V> where K:Ord, K: fmt::Display {
    m: usize,
    root: Node<K, V>,
}

impl<K, V> BTree<K, V> where K:Ord, K: fmt::Display {
    fn new(m: usize) -> BTree<K, V> {
        BTree::<K,V>::new_with(3)
    }

    fn new_with(order: usize) -> BTree<K, V> {
        BTree::<K, V> {
            m: order,
            root: Node::<K, V>::new(),
        }
    }

    fn insert(&mut self, x: K, v: V) {
        let need_split = self.root.insert(x, v, self.m);
        if need_split {
            println!("Split at Root!");
            let mut tmp = Node::new();
            std::mem::swap(&mut tmp, &mut self.root);

            let split = tmp.split(self.m);

            self.root.ks.push(split.0);
            self.root.vs.push(split.1);
            self.root.ns.push(tmp);
            self.root.ns.push(split.2);
        }
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

// Separate keys and values.
// In the search, I need to acess only keys.
// Having keys as a dedicated data structure makes
// the search faster.
// More work is required in the insert, but it does not matter
// in a usual case.
struct Node<K, V> where K:Ord {
    ks: Vec<K>,
    vs: Vec<V>,
    ns: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V> where K:Ord, K: fmt::Display {
    fn new() -> Node<K, V> {
        Node::<K, V> {
            ks: Vec::<K>::with_capacity(2),
            vs: Vec::<V>::with_capacity(2),
            ns: Vec::<Node<K, V>>::with_capacity(3),
        }
    }

    fn insert(&mut self, x: K, v: V, m: usize) -> bool {
        let pos = self.find_pos(&x);
        if self.ns.len() == 0 {
            println!("Insert A");
            if pos.1 {
                // overwrite
                self.ks[pos.0] = x;
                self.vs[pos.0] = v;
                return false;
            }
            // insert
            self.ks.insert(pos.0, x);
            self.vs.insert(pos.0, v);
            return self.ks.len() == 3;
        }

        println!("Insert B");

        let need_split = self.ns[pos.0].insert(x, v, m);
        if need_split {
            self.split_child(pos.0, m);
            return self.ks.len() == 3;
        }

        return false;
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
    fn split(&mut self, M: usize) -> (K, V, Node<K, V>) {
        println!("XXX split 000");

        let mut right = Node::new();

        let mid = M / 2;


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
}

impl<K, V> fmt::Display for Node<K, V> where K:Ord, K: fmt::Display{
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
