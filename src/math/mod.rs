struct Mat4 {
    data : [f64;16]
}

impl Mat4 { 
    pub fn new() -> Self {
        Mat4 {
            data : [0.; 16] 
        }
    }

    pub fn identity() -> Self {
        let mut result = Self::new();
        result.data[0] = 1.;
        result.data[4 + 1] = 1.;
        result.data[4 * 2 + 2] = 1.;
        result.data[4 + 3 + 3] = 1.;
        result
    }

    pub fn translate(&mut self, x : f64, y: f64, z: f64) -> () {
        self.data[12] += x;
        self.data[13] += y;
        self.data[14] += z;
    }

    pub fn stretch(&mut self, x : f64, y : f64, z: f64) -> () {
        self.data[0]         *= x;
        self.data[4 + 1]     *= y;
        self.data[4 * 2 + 2] *= z;
    }

    pub fn rotate_radians(&mut self, x : f64, y : f64, z : f64) -> () {
        
    }
}
