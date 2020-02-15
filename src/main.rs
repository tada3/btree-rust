use std::fmt;

fn main() {
    println!("Start!");
    let mut b = BTree::new(3);
    b.insert(10);
    b.insert(20);
    b.print();
    
    b.insert(30);
    b.print();

    b.insert(25);
    b.print();

    b.insert(15);
    b.print();

    b.insert(0);
    b.print();

    b.insert(35);
    b.print();

    println!("Done");

}


struct Abc {
    age: i64,
    name: String,
}

struct BTree {
    m: usize,
    root: Node,
}

impl BTree {
    fn new(m: usize) -> BTree {
        BTree {
            m: m,
            root: Node::new()
        }
    }

    fn insert(&mut self, x: i64) {
        let needSplit = self.root.insert(x);
        if needSplit {

            println!("Split at Root!");
            let mut newRoot = Node::new();
            std::mem::swap(&mut newRoot, &mut self.root);

            let split = newRoot.split();
            

            self.root.es.push(split.0);
            self.root.ns.push(newRoot);
            self.root.ns.push(split.1);
        }
    }

    fn print(&self) {
        println!("{}", self.root);

        let mut next = Vec::<&Node>::with_capacity(10);
        for n in & self.root.ns {
            next.push(n);
        }

        loop {
            if next.len() == 0 {
                break;
            }

            for n in & next {
                print!("{} ", n);
            }
            println!();

            let mut tmp = Vec::<&Node>::with_capacity(10);
            for n in & next {
               for c in & n.ns {
                    tmp.push(c);
                }
            }

            std::mem::swap(&mut tmp, &mut next);

        }
       
        

        println!();
    }

}


struct Node {
    es: Vec<i64>,
    ns: Vec<Node>,
}

fn test(x : Abc) -> Abc {
    println!("AAAAA {}, {}", x.age, x.name);
    return x;
}

impl Node {
    fn new() -> Node {
        Node {
            es: Vec::<i64>::with_capacity(2),
            ns: Vec::<Node>::with_capacity(3)
        }
    }

    fn print(&self) {
        for e in self.es.iter() {
            print!("{} ", e);
        }
        println!();
        println!();
    }

    fn insert(&mut self, x : i64) -> bool {
        let pos = self.find_pos(x);
        if self.ns.len() == 0 {
            println!("Insert A");
            if pos.1 {
                // overwrite
                self.es[pos.0] = x;
                return false;
            }
            // insert
            self.es.insert(pos.0, x);
            return self.es.len() == 3;
        }

        println!("Insert B");

        let need_split = self.ns[pos.0].insert(x);
        if need_split {
            self.split_child(pos.0);
            return self.es.len() == 3;
        }
        
        return false;
    }

    
    fn split_child(&mut self, pos:usize) {
        let result = self.ns[pos].split();
        self.es.insert(pos, result.0);
        //self.ns[pos] = result.1;
        self.ns.insert(pos + 1, result.1);
    }
    

    
    fn split(&mut self) -> (i64, Node) {
        println!("XXX split 000");
        let M = 3;
        // let mut left = Node::new();
        let mut right = Node::new();

        let mid = M / 2;

        //left.es.extend_from_slice(&self.es[0..mid]);
        //right.es.extend_from_slice(&self.es[mid+1..M]);
        right.es = self.es.split_off(mid+1);
        

        if self.ns.len() > 0 {
            //left.ns.extend_from_slice(&self.ns[0..mid]);
            //for i in 0..mid {
            //    left.ns.push(self.ns[i]);
            //}
            right.ns = self.ns.split_off(mid + 1);
 

            //for (int i = 0; i < mid; i++) {
            //    left.ns.add(ns.get(i));
            //}
            //left.ns.add(ns.get(mid));
            //left.ns.push(self.ns[mid]);


            //right.ns.extend_from_slice(self.ns[mid+1..M]);
            //for (int i = mid + 1; i < es.size(); i++) {
            //    right.ns.add(ns.get(i));
            //}
            //for i in mid+1..M {
            //    right.ns.push(self.ns[i]);
            //}
            //right.ns.push(self.ns[M]);
        }
        println!("XXX left={}", self);

        let midE = self.es.pop().unwrap();
        //return new Triple<>(midE, left, right);
       // return (midE, left, right);
       //println!("XXX left={}", self)
       println!("XXX midE={}", midE);
       println!("XXX right={}", right);
       return (midE, right);
    }
    

    fn find_pos(&self, x: i64) -> (usize, bool) {
        for i in 0..self.es.len() {
            if x < self.es[i] {
                return (i, false);
            } else if x == self.es[i] {
                return (i, true); 
            } 
        }
        return (self.es.len(), false);
    }

}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        if self.es.len() > 0 {
            write!(f, "{}", self.es[0]);
            for i in 1..self.es.len() {
                write!(f, ",{}", self.es[i]);
            }
        }
        write!(f, "]");
        Ok(())
    }
}