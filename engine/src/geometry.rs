pub struct Rect{
    x : f32,
    y : f32,
    w : f32,
    h: f32,
}

impl Rect{

    pub fn new(x:f32, y:f32, w:f32,h:f32)->Rect{
        Rect{
            x,y,w,h
        }
    }

    pub fn from_2_points(x0:f32, y0:f32, x1:f32, y1:f32)->Rect{
        Rect{
            x : x0,
            y : y0,
            w : x1-x0,
            h : y1-y0
        }
    }

    pub fn x0(&self)-> f32{
        self.x
    }

    pub fn x1(&self)-> f32{
        self.x + self.w
    }

    pub fn xc(&self)-> f32{
        (self.x0() + self.x1()) / 2.
    }

    pub fn y0(&self)-> f32{
        self.y
    }

    pub fn y1(&self)-> f32{
        self.y + self.h
    }

    pub fn yc(&self)-> f32{
        (self.y0() + self.y1()) / 2.
    }


    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn set_w(&mut self, w: f32) {
        self.w = w;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
    }

}

pub trait AsRect{
    fn as_rect(&self) -> Rect;
}