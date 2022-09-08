#![feature(generic_const_exprs)]

use dfdx::prelude::*;
use fishgen::*;

fn main() {
    type Model = (
        (Conv2D<3, 4, 3>, ReLU),
        (Conv2D<4, 8, 3>, ReLU),
        (Conv2D<8, 16, 3>, ReLU),
        FlattenImage,
        Linear<16581184, 5>,
    );

    let mut rng = rand::thread_rng();
    let mut m: Model = Default::default();
    m.reset_params(&mut rng);

    let fish_data = random_fish(1024, 1024);

    println!("generating tensor from input...");
    let input = tensor_from_data(fish_data);

    println!("running network...");
    let output: Tensor1D<5> = m.forward(input);

    println!("{output:?}");
}

fn tensor_from_data(input: FishOutput) -> Tensor3D<3, 1024, 1024> {
    let output_bytes = input.image.as_raw();
    let mut image = [[[0f32; 1024]; 1024]; 3];

    for c in 0..3 {
        for x in 0..1024 {
            for y in 0..1024 {
                image[c][y][x] = output_bytes[x * 1024 * 3 + y * 1024 + c] as f32 / 255.0;
            }
        }
    }

    TensorCreator::new(image)
}
