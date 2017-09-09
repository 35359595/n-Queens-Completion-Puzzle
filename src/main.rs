use std::io;
#[derive(PartialEq, PartialOrd, Copy, Clone)]
struct Point {
    x: i16,
    y: i16
}
#[derive(Clone)]
struct Queen {
    c: Point,
    b: Vec<Point>
}
struct Board {
    d: i16,
    s: Vec<Point>,
    q: Vec<Queen>
}
impl Point{
    pub fn new(x: i16, y: i16) -> Point{
        Point{
            x: x,
            y: y
        }
    }
}
impl Queen {
    pub fn new(p: Point) -> Queen{
        Queen{
            c: p,
            b: Vec::new()
        }
    }
    pub fn place(&mut self, b: &mut Board){
        if let Some(i) = b.point_position(&self.c) {
            b.s[i] = Point::new(0, 0);
            let mut k = 1;
            while k + &self.c.x <= b.d || k + &self.c.y <= b.d || &self.c.x - k >= 0 || &self.c.y - k >= 0 { //complete
                if &self.c.x + k <= b.d && &self.c.y + k <= b.d {
                    let pn = Point::new(&self.c.x + k, &self.c.y + k);
                    self.b.push(pn);//right down progression !complete
                    b.beat(pn);
                }
                if &self.c.x - k >= 0 && &self.c.y + k <= b.d {
                    let pn = Point::new(&self.c.x - k, &self.c.y + k);
                    self.b.push(pn);//left down progression !coplete
                    b.beat(pn);
                }
                if &self.c.x - k >= 0 && &self.c.y - k >= 0 {
                    let pn = Point::new(&self.c.x - k, &self.c.y - k);
                    self.b.push(pn);//left up progression !complete
                    b.beat(pn);
                }
                if &self.c.x + k <= b.d && &self.c.y - k <= 0 {
                    let pn = Point::new(&self.c.x + k, &self.c.y - k);
                    self.b.push(pn);//right up progression !complete
                    b.beat(pn);
                }
                if &self.c.x + k <= b.d {
                    let pn = Point::new(&self.c.x + k, &self.c.y + 0);
                    self.b.push(pn);//right straight progression !complete
                    b.beat(pn);
                }
                if &self.c.y + k <= b.d {
                    let pn = Point::new(&self.c.x + 0, &self.c.y + k);
                    self.b.push(pn);//down straight progression !complete
                    b.beat(pn);
                }
                if &self.c.x - k >= 0 {
                    let pn = Point::new(&self.c.x - k, &self.c.y + 0);
                    self.b.push(pn);//up staright progression !complete
                    b.beat(pn);
                }
                if &self.c.y - k >= 0 {
                    let pn = Point::new(&self.c.x + 0, &self.c.y - k);
                    self.b.push(pn); //left straight progression !complete
                    b.beat(pn);
                }
                k += 1;
            }
            b.q.push(self.clone());
        } 
    }
}
impl Board{
    pub fn new(d: i16) -> Board{
        Board{
            d: d,
            s: Vec::new(),
            q: Vec::new()
        }
    }
    pub fn point_position(&mut self, p: &Point) -> Option<usize> {
        let mut i = self.s.iter();
        let pos = i.position(|ref x| x == &p);
        return pos
    }
    pub fn populate(&mut self){
        for i in 1..self.d + 1 {
            for n in 1..self.d + 1{
                self.s.push(Point::new(i, n));
            }
        }
        println!("Population of the board complete.");
    }
    pub fn beat(&mut self, p: Point){
        if let Some(pos) = self.point_position(&p){
            self.s[pos] = Point::new(0, 0);
        }
    }
}
fn read_number(querry: &'static str) -> i16 {
    println!("{}", querry);
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line.");
    let inp_t = inp.trim();
    match inp_t.parse::<i16>(){
        Ok(x) => return x,
        Err(..) => panic!("Not an integger! {}", inp_t)
    };
}
fn main() {
    let size = read_number("Provide size of the board: ");
    let mut b = Board::new(size);
    b.populate();
    let mut q = Queen::new(Point::new(read_number("X of first Queen:"), read_number("Y of first Queen: ")));
    q.place(&mut b);
    let mut q2 = Queen::new(Point::new(read_number("X of second Queen:"), read_number("Y of second Queen: ")));
    q2.place(&mut b);
    let pm = b.s.clone();
    for p in pm {
        if p != Point::new(0, 0) {
            let mut qn = Queen::new(p);
            qn.place(&mut b);
        }
    }
    println!("Total Queens on board {0}x{0} is: {1} ", size, &b.q.len());
}
