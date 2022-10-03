// use nalgebra::Vector3;

// pub struct Image(Vec<Vec<Vector3>>);

// impl Image {
//     pub fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vector3) -> Image{
//         Image(
//             (0..ny)
//             .rev()
//             .map(|y| (0..nx).map(|x| f(x,y)).collect()).collect()
//         )
//     }
// }