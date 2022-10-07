extern crate nalgebra_glm as glm;

use assert_approx_eq::assert_approx_eq;


mod ray;
mod utils;
mod image2d;


use crate::ray::Ray;
use crate::utils::{lerp, luminance, rad2deg};
use crate::image2d::Image2d;

use nalgebra::{Vector3, Matrix4};

fn test_vector_and_matrices(){
    let v1 = Vector3::new(-0.1, 0.2, -0.3);
    let v2 = Vector3::new(1.0, 1.0, -1.0);
    let v3 = Vector3::new(0.5, 0.25, -0.123);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v3 = {}", v3);

    println!("You can access specific components using x, y, and z.");
    // TODO: Output the z coordinate of the normal
    println!("The z coordinate of v3 is {}.\n", v3.z);

    println!("We can also element-wise add, subtract, and multiply vectors:");
    println!("v1 + v2:\n   {}\n + {}\n = {}", v1, v2, v1 + v2);
    // TODO: divide vector 1 by vector 3
    println!("v1 / v3:\n   {}\n / {}\n = {}\n", v1, v3, v1.component_div(&v3));

    println!("or perform mixed vector-scalar arithmetic");
    println!("scalar * v2:\n   {}\n * {}\n = {}", 2.0, v2, 2.0 * v2);

    println!("We can compute the length of a vector, or normalize it, or take the dot product or cross product of two vectors:");

    println!("The length of v2 is: {}", v2.norm());
    println!("The squared length of v2 is: {}", v2.norm_squared());
    let normalized2 = v2.normalize();
    println!("A normalized copy of v2 is: {}", normalized2);
    println!("Let's confirm that its length is 1: {}\n", normalized2.norm());

    // TODO: look in vec.h to find an appropriate function to call to compute
    // the dot product and cross product between two vectors.
    println!("The dot product of v1 and v3 is: {}", glm::dot(&v1, &v3));
    println!("The cross product of v1 and v2 is: {}", glm::cross(&v1, &v2));

    // TODO: compute the angle between v1 and v3 (in degrees) using
    // either the dot or cross product. Use the rad2deg function from common.h.
    let dot: f32 = glm::dot(&v1, &v3);
    let norm = v1.norm() * v3.norm();
    let degrees: f32 = rad2deg((dot / norm).acos());

    println!("The angle between v1 and v3 is: {}", degrees);
    assert_approx_eq!(degrees, 80.0787, 1e-4);

    // We will also make use of rays, which represent an origin and a direction:
    let ray = Ray::new(
        Vector3::new(0.5, 2.0, -3.0), 
        Vector3::new(-0.25, -0.5, 0.3));

    // Let's print some info about our ray
    println!("The origin of ray is    {}.", ray.origin);
    println!("The direction of ray is {}.", ray.direction);

    // We also provide a 4x4 matrix class Mat44<T>, again templated by type T.
    // The Mat44 class includes a number of constructors. One way to fill a
    // matrix is to pass in its four *column* vectors.
    // Note that because we pass in columns, visually the matrix below looks
    // transposed: the vector {4, 5, 6, 1} is the 4th column, not row.
    let matrix = Matrix4::new(
        1., 0., 0., 0., 
        0., 2., 0., 0., 
        0., 0., 3., 0., 
        4., 5., 6., 1.
    );

    // We also provide the ability to compute matrix products and inverses.
    println!("The matrix is\n{}.", matrix);
    println!("The inverse is\n{}.", matrix.try_inverse().unwrap());
    println!("mat*inv should be the identity\n{}.", matrix * matrix.try_inverse().unwrap());
}


fn test_color_and_image()
{

    let red = Vector3::new(1., 0., 0.);
    let blue = Vector3::new(0., 0., 1.);
    let white = Vector3::new(1., 1., 1.); // This is the same as Color3f(1,1,1);

    // We can perform basic element-wise arithmatic on Colors:
    let magenta   = red + blue;
    let still_red = red.component_mul(&white);

    // TODO: Initialize the color pinkish to the average of white and red
    let mut pinkish = (white + red) / 2.0;

    println!("white    = {}.", white);
    println!("red      = {}.", red);
    println!("blue     = {}.", blue);
    println!("magenta  = {}.", magenta);
    println!("pinkish  = {}.", pinkish);
    println!("still_red = {}.", still_red);

    // We can also access the individual elements of the color by channel index:
    println!("Red channel of pinkish is: {}", pinkish[0]);

    // sincle Color3f is just a typedef for Vec3f, you can also access the channels using pinkish.x, pinkish.y,
    // pinkish.z, but this may not be very informative

    // TODO: Print out the green channel of pinkish
    println!("Green channel of pinkish is: {}", pinkish.y);
    println!("Blue channel of still_red is: {}", still_red[2]);

    pinkish[0] *= 2.;

    println!("After scaling by 2, red channel of pinkish is: {}", pinkish[0]);

    // The Color3f class provides a few additional operations which are useful
    // specifically for manipulating colors, see the bottom of the vec.h file.

    // TODO: Compute and print the luminance of pinkish. Look at vec.h to see
    // what method you might need
    println!("The luminance of pinkish is: {}", luminance(&pinkish));

    // Darts also provides the Image3f class (see image.h|cpp) to load, store,
    // manipulate, and write images.

    // Image3f is just a dynamically allocated 2D array of pixels. It
    // derives from the Array2D class, which is a generic 2D array
    // container of arbitrary size.

    // Here we construct an empty image that is 200 pixels across, and
    // 100 pixels tall:
    // auto image1 = Image3f(200, 100);
    let mut image1 = Image2d::new(200, 100);

    // In the case of Image3f, each array element (pixel) is a Color3f, which,
    // as we saw before, is itself a 3-element array.

    // We can access individual pixels of an Image3f using the (x,y) operator:
    // image1(5, 10) = white; // This sets the pixel to white
    image1[(5,10)] = white;
    
    println!("{}", white);
    // The file common.h defines a simple linear interpolation function: lerp
    // which allows us to specify two values, a and b, and an interpolation
    // parameter t. This function is a template, which means it will work with
    // any type as long as (in this case) we can add them and multiply by a
    // scalar. Just as we could interpolate between two scalar values, we can
    // also use it to interpolate between two colors:

    println!("25% of the way from blue to red is: {}.", glm::lerp(&blue, &red, 0.25));

    // Now, let's populate the colors of an entire image, and write it to a PNG
    // file.

    let mut gradient = Image2d::new(200, 100);

    // TODO: Populate and output the gradient image
    // First, loop over all rows, and then columns of an image.
    // Set the red component of a pixel's color to vary linearly from 0 at the
    // leftmost pixel to 1 at the rightmost pixel; and the green component to
    // vary from 0 at the topmost pixel to 1 at the bottommost pixel. The blue
    // component should be 0 for all pixels.

    // After populating the pixel colors, look at the member functions of
    // Image3f, and call a function to save the gradient image out to the file
    // "gradient.png".

    println!("Creating gradient image.");
    
    // for i in 0..200{
    //     for j in 0..100{
    //         gradient[(i,j)] = Vector3::new(
    //             lerp(0.0, 1.0, ((i as f32) + 0.5)/200.),
    //             lerp(0.0, 1.0, ((j as f32) + 0.5)/100.),
    //             0.);
    //     }
    // }

    for i in 0..200{
        for j in 0..100{
            gradient[(i,j)] = Vector3::new(((i as f32) + 0.5)/200., ((j as f32) + 0.5)/100., 0.);
        }
    }
    // put_your_code_here("Populate an image with a color gradient and save to \"scenes/assignment0/gradient.png\"");
    println!("Saving image \"gradient.png\" ...");

    gradient.save("scenes/assignment0/gradient.png".to_string());

    // // Now, we will load an image, modify it, and save it back to disk.
    // Image3f image;

    // // TODO: Load the image scenes/assignment0/cornellbox.png into the
    // // ``image'' variable
    // println!("Loading image cornellbox.png ...");
    // put_your_code_here("Load the image \"scenes/assignment0/cornellbox.png\".");
    // // Hint: Take a look at Image3f::load
    // // Keep in mind filenames are interpreted relative to your current
    // // working directory

    // // TODO: Convert the image to grayscale. Loop over every pixel and convert
    // // it to grayscale by replacing every pixel with its luminance
    // println!("Converting image to grayscale....");
    // put_your_code_here("Convert the image to grayscale.");

    // // TODO: Save the image to scenes/assignment0/cornell_grayscale.png
    // // Hint: Take a look at Image3f::save
    // println!("Saving image cornell_grayscale.png....");
    // put_your_code_here("Save the image to \"scenes/assignment0/cornell_grayscale.png\".");

    // success("Done!\n");
}

fn main() {
    test_vector_and_matrices();
    test_color_and_image();
}
