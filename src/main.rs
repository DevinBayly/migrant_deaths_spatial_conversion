#[derive(Debug)]
enum EL {
    None,
    Some(Box<QT>),
}
#[derive(Debug)]
struct PT {
    x:f32,
    y:f32

}
impl PT {
    fn new(x:f32,y:f32) -> Self {
        PT {
            x,y
        }
    }
}

#[derive(Debug)]
struct Rect{
    top:f32,
    bottom:f32,
    right:f32,
    left:f32,
    center:PT,
    w:f32,
    h:f32
}
// note y is up!
impl Rect {
    fn new(center:PT,w:f32,h:f32) ->Self {
        Rect{
            top: center.y + h/2.0,
            bottom:center.y - h/2.0,
            right: center.x + w/2.0,
            left : center.x - w/2.0,
            center,
            w,
            h,
        }
    }
    fn contains(&self,other:&PT) -> bool{
        other.x >= self.left && other.x < self.right && other.y >= self.bottom && other.y < self.top
    }
}
#[derive(Debug)]
struct QT {
    value: Vec<f32>,
    capacity:usize,
    bb:Rect,
    subdiv:bool,
    child: EL,
}

impl QT {
    fn addPt(&mut self, other: f32) {
        // use logic to decide if we should push or punt
        if self.value.len() < self.capacity {
            self.value.push(other);
        } else {
            if !self.subdiv {
                self.grow();
            }
            // go to the next child
            let mut ref_child = &mut self.child;
            match ref_child {
                EL::Some(b) => {
                    // change ref_child to a new child
                    b.addPt(other);
                }
                _ => {
                }
            }
        }
    }
    fn grow(&mut self) {
        // add another element at the end of the chain
        let mut ref_child = &mut self.child;
        let mut end_loop = false;
        self.subdiv = true;
        loop {

            match ref_child {
                EL::Some(b) => {
                    // change ref_child to a new child
                    ref_child = &mut b.child;
                }
                _ => {
                    *ref_child = EL::Some(Box::new(QT {
                        capacity:4,
                        subdiv:false,
                        bb:Rect::new(PT::new(self.bb.center.x - self.bb.w/4.0,self.bb.center.y),self.bb.w/2.0,self.bb.h),
                        value: vec![self.value[0] + 1f32],
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
    let width = 200.0;
    let height = 200.0;
    let mut t = QT {
        bb:Rect::new(PT::new(width/2.0,height/2.0),width,height),
        capacity:4,
        subdiv:false,
        value: vec![0f32],
        child: EL::None,
    };
    for i in 0..16 {
        println!("i is {}",i);
        t.addPt(i as f32);
    }
    println!("{:#?}",t);    
}
