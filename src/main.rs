extern crate image;


use image::{ImageBuffer, RgbImage};






// Need clone and copy to put points int an array
#[derive(Clone,Copy,Debug)]
struct Complex{
	x:f64,
	iy:f64
}

// The Mandelbrot set is the set of values of c in the complex plane for which the orbit of the critical point z = 0 under iteration of the quadratic map
// zn + 1 = zn^2 + c 
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

fn main() {
	
    println!("Rendering an image of the Mandelbrot set...");
	
	const X_RESOLUTION:u32 = 500;
	const Y_RESOLUTION:u32 = 500;
	const IMAGE_SIZE:u32 = X_RESOLUTION * Y_RESOLUTION;
	let max_iterations:usize=255;
	
	// The square  out of the complex plane we're going to map over:
	let left:f64 = -2.5;
	let right:f64 = 1.0;
	let top:f64 = 1.0;
	let bottom:f64 = -1.0;
	
	let width = right-left;
	let height = top-bottom;
	
	

	// The picture goes in here
	
    let mut image: RgbImage = ImageBuffer::new(X_RESOLUTION, Y_RESOLUTION);
       	
		
	// Making a separate array for the complex plane so we can "debug" the
	// way the Mandelbrot function works after we render the image.
	let mut plane =[Complex{x:0.0,iy:0.0}; IMAGE_SIZE as usize];
	
	// Track amount of area within the mapped plane that's not  in the Mandelbrot set
	let mut escapes = 0;
	
	// sample points from the complex plane to match the image RESOLUTION		
	for column in 0..X_RESOLUTION-1{			
		let x = left + width * (column as f64 / X_RESOLUTION as f64);		
		for row in 0..Y_RESOLUTION-1{
			let y = top - height * (row as f64 / Y_RESOLUTION as f64);			
			let point = Complex {x:x as f64, iy:y as f64};					
			let loc:usize = column as usize + row as usize  * Y_RESOLUTION as usize;
			plane[loc]=point;
			let mut iterations = in_mandelbrot(point, max_iterations);
			if iterations>255{
				iterations=255;
			}
			let color:u8 = iterations as u8;
			

			
			*image.get_pixel_mut(column,row) = image::Rgb([color,color,color]);
			if iterations < max_iterations {
				escapes += 1;
			}
			//println!("Point {} {} in set ? {}",point.x, point.iy,image[loc]);
		}
	}
	let percent_outside:i32 = (escapes as f64  * 100.0/ IMAGE_SIZE as f64)  as i32;
	println!("Escapes: {}, percent outside within image: {}",escapes,percent_outside);
	image.save("output.png").unwrap();

}
