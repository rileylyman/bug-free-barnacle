#![allow(dead_code)]

use packed_simd::{
    f32x4,
    m32x4,
    cptrx4,
};

use std::ops::Mul;

const MASK3T1F : m32x4 = m32x4::new(true, true, true, false);
const MASK4T   : m32x4 = m32x4::new(true, true, true, true);

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub struct Mat4 {
    data  : [f32;16],
    rows  : [f32x4;4],
    dirty : bool,
}

impl Mat4 { 
    pub fn new() -> Self {
        Mat4 {
            data   : [0.; 16],
            rows   : [f32x4::splat(0.);4],
            dirty  : false,
        }
    }

    pub fn from_data(data: [f32;16]) -> Self {
        Mat4 {
            data  : data,
            rows  : [
                f32x4::from_slice_unaligned(&data[..4]),
                f32x4::from_slice_unaligned(&data[4..8]),
                f32x4::from_slice_unaligned(&data[8..12]),
                f32x4::from_slice_unaligned(&data[12..]),
            ],
            dirty : false,
        }
    }

    pub fn identity() -> Self {
        Mat4 {
            data : [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
            rows : [
                f32x4::new(1.0, 0.0, 0.0, 0.0),
                f32x4::new(0.0, 1.0, 0.0, 0.0),
                f32x4::new(0.0, 0.0, 1.0, 0.0),
                f32x4::new(0.0, 0.0, 0.0, 1.0),
            ],
            dirty  : false,
        }
    }
    
    pub fn get(&mut self) -> &[f32] {
        if self.dirty {
            self.flush_rows();
        }
        &self.data[..]
    }

    pub fn flush_rows(&mut self) -> () {
        self.rows[0].write_to_slice_aligned(&mut self.data[0..4]);
        self.rows[1].write_to_slice_aligned(&mut self.data[4..8]);
        self.rows[2].write_to_slice_aligned(&mut self.data[8..12]);
        self.rows[3].write_to_slice_aligned(&mut self.data[12..]);
        self.dirty = false;
    }

    pub fn translate(&mut self, x : f32, y: f32, z: f32) -> &mut Self {
        //TODO: Incorrect
        let vec = f32x4::new(x, y, z, 0.0); 
        self.rows[3] += vec;
        self.dirty = true;
        self
    }

    pub fn stretch(&mut self, x : f32, y : f32, z: f32) -> &mut Self {
        //@Inefficient?
        self.rows[0] = self.rows[0].replace(0, self.rows[0].extract(0) * x);
        self.rows[1] = self.rows[1].replace(1, self.rows[1].extract(1) * y);
        self.rows[2] = self.rows[2].replace(2, self.rows[2].extract(2) * z);
        self.dirty = true;
        self
    }

    pub fn rotate_radians(self, radians: f32, axis: Axis) -> Self { 
        match axis {
            Axis::X => {
                x_axis_rotation(radians) * self
            },
            Axis::Y => {
                y_axis_rotation(radians) * self
            },
            Axis::Z => {
                z_axis_rotation(radians) * self
            },
        }
    }
}

const X_ROT_DAT: [f32;16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];
fn x_axis_rotation(rads: f32) -> Mat4 {
    let mut x_rot = X_ROT_DAT;
    x_rot[5] = rads.cos();
    x_rot[6] = -rads.sin();
    x_rot[9] = -x_rot[6];
    x_rot[10] = x_rot[5];
    Mat4::from_data(x_rot)
}

const Y_ROT_DAT: [f32;16] = [
    0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];
fn y_axis_rotation(rads: f32) -> Mat4 {
    let mut y_rot = Y_ROT_DAT;
    y_rot[0] = rads.cos();
    y_rot[2] = rads.sin();
    y_rot[8] = -y_rot[2];
    y_rot[10] = y_rot[0];
    Mat4::from_data(y_rot)
}

const Z_ROT_DAT: [f32;16] = [
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];
fn z_axis_rotation(rads: f32) -> Mat4 {
    let mut z_rot = Z_ROT_DAT;
    z_rot[0] = rads.cos();
    z_rot[1] = -rads.sin();
    z_rot[4] = -z_rot[1];
    z_rot[5] = z_rot[0];
    Mat4::from_data(z_rot)
}

impl Mul for Mat4 {
    type Output = Self;

    //Note: the right matrix must be flushed
    //@Inefficient: Need strided loads!!!!!!!!!
    fn mul(self, right: Self) -> Self {
        let rcols = unsafe {
            let default = f32x4::splat(0.);
            let ptr0 = cptrx4::new(
                &right.data[0],
                &right.data[4],
                &right.data[8],
                &right.data[12],
            );
            let ptr1 = cptrx4::new(
                &right.data[1],
                &right.data[5],
                &right.data[9],
                &right.data[13],
            );
            let ptr2 = cptrx4::new(
                &right.data[2],
                &right.data[6],
                &right.data[10],
                &right.data[14],
            );
            let ptr3 = cptrx4::new(
                &right.data[3],
                &right.data[7],
                &right.data[11],
                &right.data[15],
            );
            [
                ptr0.read(MASK4T, default),
                ptr1.read(MASK4T, default),
                ptr2.read(MASK4T, default),
                ptr3.read(MASK4T, default),
            ]
        };
        let mut mul_data = [0.;16];
        for row in 0..4 {
            mul_data[row * 4]     = (self.rows[row] * rcols[0]).sum();
            mul_data[row * 4 + 1] = (self.rows[row] * rcols[1]).sum();
            mul_data[row * 4 + 2] = (self.rows[row] * rcols[2]).sum();
            mul_data[row * 4 + 3] = (self.rows[row] * rcols[3]).sum();
        }
        Mat4::from_data(mul_data)
    }
}
