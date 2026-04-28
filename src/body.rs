pub struct Body {
    mass: f64,
    height: f64,
}

/*Would shorten but that will defeat original intent to learn Struct & Impl */
impl Body {
    pub fn new(mass:f64, height:f64) -> Self {
        Body {
            mass,
            height
        }
    }
   pub fn bmi(&self) -> f64 {
        (self.mass) / (self.height * self.height)
    }
}