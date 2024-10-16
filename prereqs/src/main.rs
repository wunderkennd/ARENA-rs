use candle_einops::einops;
use candle_core::{Device, Result, read_npy};
use arena_utils::candle_utils::{display_array_as_img, load_numbers};
use std::path::Path;


fn main() -> Result<()> {
    let device = Device::Cpu;
    let numbers_path = Path::new("numbers.npy");
    let arr = load_numbers(numbers_path)?;

    // Example 1
    let arr1 = einops!("b c h w -> c h (b w)", &arr);
    println!("{:?}", display_array_as_img(&arr1));

    // Example 2
    // let arr2 = einops!("c h w -> c (2 h) w", &arr.slice((0..1, .., .., ..)));
    // display_array_as_img(&arr2)?;

    // Example 3
    let arr3 = einops!(
        "b c h w -> c (b h) (2 w)",
        &arr
    );
    println!("{:?}", display_array_as_img(&arr3));

    // // Example 4
    // let arr4 = einops!("c h w -> c (h 2) w", &arr.slice((0..1, .., .., ..))?)?;
    // display_array_as_img(&arr4)?;
    //
    // // Example 5
    // let arr5 = einops!("c h w -> h (c w)", &arr.slice((0..1, .., .., ..))?)?;
    // display_array_as_img(&arr5)?;
    //
    // // Example 6
    // let arr6 = einops!("(b1 b2) c h w -> c (b1 h) (b2 w)", &arr)?;
    // display_array_as_img(&arr6)?;
    //
    // // Example 7
    // let arr7 = einops!("b c h w -> h (b w)", &arr);
    // let arr7 = arr7.max(0)?;
    // display_array_as_img(&arr7)?;
    //
    // // Example 8
    // let arr8 = einops!("b c h w -> h w", &arr);
    // let arr8 = arr8.min(0)?;
    // display_array_as_img(&arr8)?;
    //
    // // Example 9
    // let arr9 = einops!("c h w -> c w h", &arr.slice((1..2, .., .., ..))?)?;
    // display_array_as_img(&arr9)?;
    //
    // // Example 10
    // let arr10 = einops!(
    //     "(b1 b2) c (h h2) (w w2) -> c (b1 h) (b2 w)",
    //     &arr,
    //     Some(&[("h2", 2), ("w2", 2), ("b1", 2)]),
    // )?;
    // let arr10 = arr10.max(0)?;
    // display_array_as_img(&arr10)?;

    Ok(())
}
