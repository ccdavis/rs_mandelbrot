extern crate image;


use image::{ImageBuffer, RgbImage};
use std::ops::{Add, Sub, Mul};


#[derive(Clone,Copy,Debug, PartialEq)]
struct Complex{
	x:f64,
	iy:f64
}


	
impl Add for Complex {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {x: self.x + other.x, iy: self.iy + other.iy}
	}
}

impl Mul for Complex {
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		let mut new_x = self.x * other.x; // first
		let mut new_iy=		self.x * other.iy; // outers
		new_iy = new_iy + self.iy * other.x;  // inners		
		new_x = new_x + self.iy * other.iy * -1.0; // last
		
		Self { x:new_x, iy:new_iy}					
	}
}

impl Complex {	

	fn square(self : Complex)->Complex{				
		self * self
	}
	
	// If the sums of the squares of the real and imaginary parts exceed 4, i.e.
	// the distance from the center > 2. You could use a generic abs(Complex) > 2 method.
	fn pythagorean_escape(&self)->bool{
		(self.x * self.x + self.iy*self.iy)>=4.0
	}
	
	// Using the complex number operations we defined, this won't be 
	// as efficient as could be but is clear code.
	fn in_mandelbrot(self, max_iterations:usize)->usize{
		let mut iterations:usize = 0;
		let seed = Complex {x:0.0,iy:0.0};		 // change seed to generate other images
		
		let c = self;
		let mut zn = seed;
		
		while !zn.pythagorean_escape() && iterations < max_iterations{
			zn =  zn*zn + c;
			iterations += 1;			
		}
		iterations
	}
}



// The Mandelbrot set is the set of values of c in the complex plane for which the orbit of the critical point z = 0 under iteration of the quadratic map
// zn + 1 = zn^2 + c 
// This is the simplified ccalculation reducing the number of multiplications.
fn in_mandelbrot(c:Complex, max_iterations:usize)->usize{	
	let mut iteration:usize = 0;	
	let mut zn = Complex {x:0.0,iy:0.0};
	
	// check that the iteration doesn't escape
	while zn.x * zn.x + zn.iy * zn.iy <= 4.0 && iteration < max_iterations {
		// perform multiplication and addition on complex numbers	 zn and c
		let  tmp_x = zn.x * zn.x - zn.iy * zn.iy + c.x;
		zn.iy = 2.0 * zn.x * zn.iy + c.iy;
		zn.x = tmp_x;
		iteration += 1;
	}
	iteration		
}


// left,right,top,bottom are locations in the complex plan i.e. the viewport
// Standard starting view is  1.0, -2.5, 1.0, -1.0
fn render(x_resolution:u32, y_resolution:u32,left:f64,top:f64, right:f64, bottom:f64){
	let image_size = x_resolution * y_resolution;
	let max_iterations:usize=255;
		
	let width = right-left;
	let height = top-bottom;
	
	// The picture goes in here	
    let mut image: RgbImage = ImageBuffer::new(x_resolution, y_resolution);
       	
	// Track amount of area within the mapped plane that's not  in the Mandelbrot set
	let mut escapes = 0;
	
	// sample points from the complex plane to match the image RESOLUTION		
	for column in 0..x_resolution-1{			
		let x = left + width * (column as f64 / x_resolution as f64);		
		for row in 0..y_resolution-1{
			let y = top - height * (row as f64 / y_resolution as f64);			
			let point = Complex {x:x as f64, iy:y as f64};					
			let mut iterations = point.in_mandelbrot(max_iterations);
			//let mut iterations=in_mandelbrot(point,max_iterations);
			if iterations>255{
				iterations=255;
			}
			let color:u8 = iterations as u8;
			
			*image.get_pixel_mut(column,row) = image::Rgb([color,color,color]);
			if iterations < max_iterations {
				escapes += 1;
			}			
		}
	}
	let percent_outside:i32 = (escapes as f64  * 100.0/ image_size as f64)  as i32;
	println!("Escapes: {}, percent outside within image: {}",escapes,percent_outside);
	image.save("mandelbrot.png").unwrap();
}
	


fn main() {
	
    println!("Rendering an image of the Mandelbrot set...");
	
	const X_RESOLUTION:u32 = 500;
	const Y_RESOLUTION:u32 = 500;

	// The square  out of the complex plane we're going to map over:
	const LEFT:f64 = -2.5;
	const RIGHT:f64 = 1.0;
	const TOP:f64 = 1.0;
	const BOTTOM:f64 = -1.0;
	
	render(X_RESOLUTION, Y_RESOLUTION, LEFT, TOP, RIGHT, BOTTOM);
}	
	