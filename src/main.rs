use rand::prelude::*;


#[derive(Debug)]
enum EL {
    None,
    Some(Box<QT>),
}
#[derive(Debug,Clone,Copy)]
struct PT {
    x: f32,
    y: f32,
}
impl PT {
    fn new(x: f32, y: f32) -> Self {
        PT { x, y }
    }
}

#[derive(Debug)]
struct Rect {
    top: f32,
    bottom: f32,
    right: f32,
    left: f32,
    center: PT,
    w: f32,
    h: f32,
}
// note y is up!
impl Rect {
    fn new(center: PT, w: f32, h: f32) -> Self {
        Rect {
            top: center.y + h / 2.0,
            bottom: center.y - h / 2.0,
            right: center.x + w / 2.0,
            left: center.x - w / 2.0,
            center,
            w,
            h,
        }
    }
    fn contains(&self, other: &PT) -> bool {
        other.x >= self.left && other.x < self.right && other.y >= self.bottom && other.y < self.top
    }
}
#[derive(Debug)]
struct QT {
    values: Vec<PT>,
    capacity: usize,
    bb: Rect,
    subdiv: bool,
    // children organized by cardinal points
    ne_child: EL,
    nw_child: EL,
    sw_child: EL,
    se_child: EL,
}

impl QT {
    fn new(rect: Rect, cap: usize) -> Self {
        //
        Self {
            bb: rect,
            capacity: cap,
            subdiv: false,
            values: vec![],
            ne_child: EL::None,
            nw_child: EL::None,
            sw_child: EL::None,
            se_child: EL::None,
        }
    }
    fn query(&self,other:PT) -> Vec<PT> {
        // find the child that contains our point, and then call query again until we have a non subdivided one
        let mut res = vec![];
        // check ne
        match &self.ne_child {
            EL::Some(b)=> {
                if b.bb.contains(&other) {
                    // check for subdivision
                    if !b.subdiv {
                        // add the group to our res
                        res.extend(self.values.clone());
                    } else {
                        // query the children of b
                        res.extend(b.query(other)); 
                    }
                }
            },
            _=> {}
        }
        // check nw
        match &self.nw_child {
            EL::Some(b)=> {
                if b.bb.contains(&other) {
                    // check for subdivision
                    if !b.subdiv {
                        // add the group to our res
                        res.extend(self.values.clone());
                    } else {
                        // query the children of b
                        res.extend(b.query(other)); 
                    }
                }
            },
            _=> {}
        }
        // check se
        match &self.se_child {
            EL::Some(b)=> {
                if b.bb.contains(&other) {
                    // check for subdivision
                    if !b.subdiv {
                        // add the group to our res
                        res.extend(self.values.clone());
                    } else {
                        // query the children of b
                        res.extend(b.query(other)); 
                    }
                }
            },
            _=> {}
        }
        // check sw
        match &self.sw_child {
            EL::Some(b)=> {
                if b.bb.contains(&other) {
                    // check for subdivision
                    if !b.subdiv {
                        // add the group to our res
                        res.extend(self.values.clone());
                    } else {
                        // query the children of b
                        res.extend(b.query(other)); 
                    }
                }
            },
            _=> {}
        }

        return res;
    }
    fn addPt(&mut self, other: PT) {
        // if it doesn't contain the point don't do anything
        if !self.bb.contains(&other) {
            return 
        }
        // use logic to decide if we should push or punt
        if self.values.len() < self.capacity {
            self.values.push(other);
        } else {
            if !self.subdiv {
                self.subdivide();
            }
            // go through the children and add the point to the correct oneto the next child
            // ne
            let mut ref_child = &mut self.ne_child;
            Self::add_to_child(ref_child, other);
            // nw
            let mut ref_child = &mut self.nw_child;
            Self::add_to_child(ref_child, other);
            // se
            let mut ref_child = &mut self.se_child;
            Self::add_to_child(ref_child, other);
            // sw
            let mut ref_child = &mut self.sw_child;
            Self::add_to_child(ref_child, other);
            //match ref_child {
            //    EL::Some(b) => {
            //        // change ref_child to a new child
            //        b.addPt(other);
            //    }
            //    _ => {
            //    }
            //}
        }
    }
    fn add_to_child(child: &mut EL, other: PT) {
        match child {
            EL::Some(b) => b.addPt(other),
            _ => {}
        };
    }
    fn subdivide(&mut self) {
        // add another element at the end of the chain
        // in subdivide we should populate all the cardinal children we don't have to loop because it will get called on the correct unsubdivided element anyways
        self.subdiv = true;
        self.ne_child = EL::Some(Box::new(QT::new(
            Rect::new(
                PT::new(
                    self.bb.center.x - self.bb.w / 4.0,
                    self.bb.center.y + self.bb.h / 4.0,
                ),
                self.bb.w / 2.0,
                self.bb.h / 2.0,
            ),
            4,
        )));
        self.nw_child = EL::Some(Box::new(QT::new(
            Rect::new(
                PT::new(
                    self.bb.center.x + self.bb.w / 4.0,
                    self.bb.center.y + self.bb.h / 4.0,
                ),
                self.bb.w / 2.0,
                self.bb.h / 2.0,
            ),
            4,
        )));
        self.se_child = EL::Some(Box::new(QT::new(
            Rect::new(
                PT::new(
                    self.bb.center.x - self.bb.w / 4.0,
                    self.bb.center.y - self.bb.h / 4.0,
                ),
                self.bb.w / 2.0,
                self.bb.h / 2.0,
            ),
            4,
        )));
        self.sw_child = EL::Some(Box::new(QT::new(
            Rect::new(
                PT::new(
                    self.bb.center.x + self.bb.w / 4.0,
                    self.bb.center.y - self.bb.h / 4.0,
                ),
                self.bb.w / 2.0,
                self.bb.h / 2.0,
            ),
            4,
        )));
        // add points to children and then set to []
        for value in self.values.iter() {
            // ne
            let mut ref_child = &mut self.ne_child;
            Self::add_to_child(ref_child, *value);
            // nw
            let mut ref_child = &mut self.nw_child;
            Self::add_to_child(ref_child, *value);
            // se
            let mut ref_child = &mut self.se_child;
            Self::add_to_child(ref_child, *value);
            // sw
            let mut ref_child = &mut self.sw_child;
            Self::add_to_child(ref_child, *value);
            //match ref_child {
        }
        self.values = vec![];
    }
}

fn main() {
    let width = 200.0;
    let height = 200.0;
    // make a new quad tree
    // t
    let mut t = QT::new(Rect::new(PT::new(100.0,100.0),200.0,200.0),4);
    let mut rng = thread_rng();
    for i in 0..16 {
        let pt = PT::new(rng.gen::<f32>()*200.0,rng.gen::<f32>()*200.0);
        println!("i is {}", i);
        t.addPt(pt);
    }
    println!("{:#?}", t);
    println!("querying {:?}",t.query(PT::new(10.0,10.0)));
}
