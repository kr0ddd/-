pub struct A {
    pub list: Vec<A>,
    pub v: i32,
}

impl A {
    pub fn Upt(&mut self, list: Vec<A>, v: i32) {
        self.list = list;
        self.v = v;
    }
}

pub fn main() {
    let mut list: Vec<A> = Vec::new();
    list.push(A {
        list: Vec::new(),
        v: 1,
    });
    list.push(A {
        list: Vec::new(),
        v: 2,
    });
    for c in list.iter() {
        let a = A {
            list: Vec::new(),
            v: 2,
        };
        //⬇️这里报错 cannot borrow as mutable//move out of `list` occurs here
        a.Upt(list, 1);
    }
}
