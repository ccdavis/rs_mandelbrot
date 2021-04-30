extern crate image;
// The Mandelbrot set is the set of values of c in the complex plane for which the orbit of the critical point z = 0 under iteration of the quadratic map
// zn + 1 = zn^2 + c 

use image::{ImageBuffer, RgbImage};
use std::ops::{Add, Sub, Mul};
use  rayon;	
use rayon::prelude::*;
use std::time::Duration;


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
//
// This is the simplified ccalculation reducing the number of multiplications.
// This does the same thing as  Complex::in_mandelbrot() but slightly faster
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

fn compute_vertical_line(x:f64, y_resolution:u32,top:f64, height:f64)->Vec<usize>{
	let max_iterations:usize=1500;	
	let mut line:Vec<usize> = Vec::new();
	for row in 0..y_resolution-1{
		let y = top - height * (row as f64 / y_resolution as f64);			
		let point = Complex {x:x as f64, iy:y as f64};					
		let  iterations = point.in_mandelbrot(max_iterations);
		//let  iterations=in_mandelbrot(point,max_iterations);
		line.push(iterations);
	}		
	line	
}

fn compute_image(left:f64,top:f64,right:f64,bottom:f64,x_resolution:u32,y_resolution:u32)->Vec<Vec<usize>>{
	let width = right-left;
	let height = top-bottom;
		   	
	// sample points from the complex plane to match the image RESOLUTION			
	// parallel iteration
	let data = (0..x_resolution-1).into_par_iter().map(|column|{
		let x = left + width * (column as f64 / x_resolution as f64);		
		return compute_vertical_line(x, y_resolution, top, height)
	}).collect();
	data
}

// Figure out the colors for the pixel and place it in the image buffer
fn plot_pixel(image:& mut RgbImage,column:u32, row:u32,iterations:usize){
	let color = (iterations % 256) as u8;
	let default_colors = [color,color,color];
	
	let colors = match color {
		1 => [0,0,0],
		2 => [80,0,0],
		3 => [125,0,0],
		4..=8 => [195,50,50],
		9..=20 =>[255,150,10],
		21..=25 => [15,180,5],
		26..=50 =>[50,25,250],
		51..=95 => [255,0,255],
		_ => default_colors							
	};
	
	*image.get_pixel_mut(column,row) = image::Rgb(colors);	
}

// Standard starting view is  1.0, -2.5, 1.0, -1.0
// So with square pixels you'd want about 3.5:2 horizontal:vertical aspect ratio.
fn render(x_resolution:u32, y_resolution:u32,data:Vec<Vec<usize>>){
	// The picture goes in here	
    let mut image: RgbImage = ImageBuffer::new(x_resolution, y_resolution);
    			
	for column in 0..x_resolution-1{
		for row in 0..y_resolution-1{			
			let iterations = data[column as usize][row as usize];
			plot_pixel(& mut image, column as u32, row as u32, iterations);						
		}
	}
	image.save("mandelbrot.png").unwrap();
}
	


fn main() {
	
    println!("Rendering an image of the Mandelbrot set...");
	
	const X_RESOLUTION:u32 = 1400;
	const Y_RESOLUTION:u32 = 800;
	let image_size = X_RESOLUTION * Y_RESOLUTION;

	// The square  out of the complex plane we're going to map over:
	const LEFT:f64 = -2.5;
	const RIGHT:f64 = 1.0;
	const TOP:f64 = 1.0;
	const BOTTOM:f64 = -1.0;
	
	let data = compute_image(LEFT, TOP, RIGHT, BOTTOM, X_RESOLUTION, Y_RESOLUTION);	
	render(X_RESOLUTION, Y_RESOLUTION, data);
	println!("Rendered ({} + i{}), ({} + i{}) with {} pixels",LEFT,TOP,BOTTOM,RIGHT,image_size);	
}	
	