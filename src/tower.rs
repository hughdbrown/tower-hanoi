use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct DiskMove {
    src: char,
    dst: char,
}

impl fmt::Display for DiskMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move disk from {} to {}", self.src, self.dst)
    }
}

pub struct Tower {
    moves: Vec<DiskMove>,
    num_disks: i8,
}

impl Tower {
    pub fn new(n: i8) -> Self {
        Tower { moves: Vec::<DiskMove>::new(), num_disks: n }
    }
    
    fn move_disks(self: &mut Self, disks: i8, src: char, dst: char, tmp: char) {
        if disks > 0 {
            self.move_disks(disks - 1, src, tmp, dst);
            self.moves.push(DiskMove{src, dst});
            self.move_disks(disks - 1, tmp, dst, src);
        }
    }

    pub fn run(&mut self) {
        let disks = self.num_disks;
        self.move_disks(disks, 'A', 'B', 'C');
    }

    fn draw(&self, a: &[i8], b: &[i8], c: &[i8]) {
        println!("-----------------");
        println!("A: {:?}", a);
        println!("B: {:?}", b);
        println!("C: {:?}", c);
    }

    pub fn visualize(self) {
        let mut a: Vec<i8> = vec![];
        let mut b: Vec<i8> = vec![];
        let mut c: Vec<i8> = vec![];
        for i in (0..self.num_disks).rev() {
            a.push(i);
        }
        self.draw(&a, &b, &c);
        for diskmove in self.moves.iter() {
            if let Some(val) = if diskmove.src == 'A' {a.pop()} else if diskmove.src == 'B' {b.pop()} else {c.pop()} {
                if diskmove.dst == 'A' {
                    a.push(val);
                } else if diskmove.dst == 'B' {
                    b.push(val);
                } else {
                    c.push(val);
                }
            }
            self.draw(&a, &b, &c);
        }
    }
}

