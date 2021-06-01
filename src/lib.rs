use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Complex {
    real: f64,
    imag: f64,
}

// struct for complex numbers
fn build_complex(real: f64, imag: f64) -> Complex {
    Complex {
        real,
        imag,
    }
}

// methods for complex numbers
impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        build_complex(self.real + other.real, self.imag + other.imag)
    }

    fn mag(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    fn mul(&self, other: &Complex) -> Complex {
        let real_part = self.real*other.real - self.imag*other.imag;
        let imag_part = self.imag*other.real + self.real*other.imag;
        build_complex(real_part, imag_part)
    }
}

fn mandelbrot(cplx: &Complex) -> bool {
    const ITER_CONST: i32 = 100;
    let z = build_complex(0.0, 0.0);

    let in_set = false;
    for count in 0..ITER_CONST {
        let z = (z.mul(&z)).add(&cplx); // z = z^2 + cplx
        if z.mag() > 2.0 {
            break;
        }
        if count > ITER_CONST {
            let in_set = true;
            break;
        }
    }
    in_set
}
/*
fn initialize_points(x_len: i32, y_len: i32) ->  {
    let mut points_array = 
}*/

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.fill_text("Hello World", 10.0, 50.0);

 /*   // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
*/
}