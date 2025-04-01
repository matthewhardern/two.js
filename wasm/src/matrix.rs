// wasm/src/matrix.rs
use wasm_bindgen::prelude::*;
use glam::Mat3;

#[wasm_bindgen]
pub struct WasmMatrixEngine {
    matrix: Mat3,
}

#[wasm_bindgen]
impl WasmMatrixEngine {
    // Default constructor (identity matrix)
    #[wasm_bindgen(constructor)]
    pub fn new(elements: Box<[f32]>) -> Self {
        Self {
            matrix: Mat3::IDENTITY,
        }
    }

    // Constructor with elements
    #[wasm_bindgen(js_name = fromElements)]
    pub fn from_elements(
        a: f32, b: f32, c: f32,
        d: f32, e: f32, f: f32,
        g: f32, h: f32, i: f32
    ) -> Self {
        // Create from Two.js format (row-major)
        Self {
            matrix: Mat3::from_cols_array(&[
                a, d, g,
                b, e, h,
                c, f, i
            ]),
        }
    }

    // Constructor from array
    #[wasm_bindgen(js_name = fromArray)]
    pub fn from_array(elements: Box<[f32]>) -> Self {
        if elements.len() >= 9 {
            return Self::from_elements(
                elements[0], elements[1], elements[2],
                elements[3], elements[4], elements[5],
                elements[6], elements[7], elements[8]
            );
        }
        // Default to identity if not enough elements
        Self::new()
    }

    #[wasm_bindgen]
    pub fn set(&mut self,
               a: f32, b: f32, c: f32,
               d: f32, e: f32, f: f32,
               g: f32, h: f32, i: f32) -> Box<[f32]> {
        // Update internal matrix
        self.matrix = Mat3::from_cols_array(&[
            a, d, g,
            b, e, h,
            c, f, i
        ]);

        // Return the elements in Two.js format for convenience
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn identity(&mut self) -> Box<[f32]> {
        self.matrix = Mat3::IDENTITY;
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn multiply(&mut self,
                    a: f32, b: f32, c: f32,
                    d: f32, e: f32, f: f32,
                    g: f32, h: f32, i: f32) -> Box<[f32]> {
        let other = Mat3::from_cols_array(&[
            a, d, g,
            b, e, h,
            c, f, i
        ]);

        self.matrix = self.matrix * other;
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn translate(&mut self, x: f32, y: f32) -> Box<[f32]> {
        let translate = Mat3::from_translation(glam::Vec2::new(x, y));
        self.matrix = self.matrix * translate;
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn rotate(&mut self, radians: f32) -> Box<[f32]> {
        let rotate = Mat3::from_rotation_z(radians);
        self.matrix = self.matrix * rotate;
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn scale(&mut self, x: f32, y: Option<f32>) -> Box<[f32]> {
        let y = y.unwrap_or(x);
        let scale = Mat3::from_scale(glam::Vec2::new(x, y));
        self.matrix = self.matrix * scale;
        self.to_array()
    }

    #[wasm_bindgen(js_name = skewX)]
    pub fn skew_x(&mut self, radians: f32) -> Box<[f32]> {
        let tan_val = radians.tan();
        let skew = Mat3::from_cols_array(&[
            1.0, 0.0, 0.0,
            tan_val, 1.0, 0.0,
            0.0, 0.0, 1.0
        ]);
        self.matrix = self.matrix * skew;
        self.to_array()
    }

    #[wasm_bindgen(js_name = skewY)]
    pub fn skew_y(&mut self, radians: f32) -> Box<[f32]> {
        let tan_val = radians.tan();
        let skew = Mat3::from_cols_array(&[
            1.0, tan_val, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ]);
        self.matrix = self.matrix * skew;
        self.to_array()
    }

    #[wasm_bindgen]
    pub fn inverse(&self) -> Box<[f32]> {
        // Get the Mat3 from glam
        let inverse_result: Mat3 = self.matrix.inverse();
        let cols = inverse_result.to_cols_array();
        [
            cols[0], cols[3], cols[6],
            cols[1], cols[4], cols[7],
            cols[2], cols[5], cols[8]
        ].into()
    }

    #[wasm_bindgen]
    pub fn multiply_vector(&self, x: f32, y: f32, z: Option<f32>) -> Box<[f32]> {
        let z = z.unwrap_or(1.0);
        let v = glam::Vec3::new(x, y, z);
        let result = self.matrix * v;

        [result.x, result.y, result.z].into()
    }

    // Helper method to convert to Two.js format
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Box<[f32]> {
        let cols = self.matrix.to_cols_array();
        [
            cols[0], cols[3], cols[6],
            cols[1], cols[4], cols[7],
            cols[2], cols[5], cols[8]
        ].into()
    }
}