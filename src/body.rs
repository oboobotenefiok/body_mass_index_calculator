pub struct Body {
    mass: f64,
    height: f64, // Remove this comma ,
                 // I removed it but noticed cargo fmt keeps adding it back. Cool though. Same wuth match.
}

/* Would shorten but that will defeat original intent to learn Struct & Impl */
impl Body {
    pub fn new(mass: f64, height: f64) -> Self {
        // I just changed this from Body to Self and it feels so good :-)
        //   Body { mass, height }
        Self { mass, height }
    }
    pub fn bmi(&self) -> f64 {
        (self.mass) / /* (self.height * self.height) Cause I found a method for it */
        self.height.powf(2.0) // Still float, powi is for integers
    }
}
