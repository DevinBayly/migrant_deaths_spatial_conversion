#[derive(Debug)]
enum EL {
    None,
    Some(Box<Test>),
}

#[derive(Debug)]
struct Test {
    value: Vec<f32>,
    child: EL,
}

impl Test {
    fn addPt(&mut self, other: f32) {
        self.value.push(other);
    }
    fn grow(&mut self) {
        // add another element at the end of the chain
        let mut ref_child = &mut self.child;
        let mut end_loop = false;
        loop {

            match ref_child {
                EL::Some(b) => {
                    // change ref_child to a new child
                    ref_child = &mut b.child;
                }
                _ => {
                    *ref_child = EL::Some(Box::new(Test {
                        value: vec![1f32],
                        child: EL::None,
                    }));
                    end_loop = true;
                }
            }
            if end_loop {
                break;
            }
        }
    }
}

fn main() {
    let mut t = Test {
        value: vec![0f32],
        child: EL::None,
    };
    t.grow();
    println!("{:?}", t);
    t.grow();
    println!("{:?}", t);
}
