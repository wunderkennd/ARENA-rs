use candle_core::{Tensor, Device, Result, DType};
use candle_core::npy::*;
use std::fs::File;
use std::path::Path;
use ndarray_npy::read_npy;

pub fn load_numbers(path: &Path) -> Result<Tensor> {
    let mut file = File::open(path).map_err(candle_core::Error::Io)?;
    let header = (&mut file)?;
    let matrix = read_npy(header).map_err(candle_core::Error::Io);
    matrix

    // let mut npz = NpzReader::new(file).map_err(|e| candle_core::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    // let arr: ndarray::Array<f32, ndarray::IxDyn> = npz.by_name("arr_0").map_err(|e| candle_core::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    // let shape = arr.shape().to_vec();
    // Tensor::from_vec(arr.into_raw_vec(), shape, &Device::Cpu)
}

pub fn display_array_as_img(img_array: &Tensor) -> Result<()> {
    let shape = img_array.shape();
    assert!(shape.dims().len() == 2 || (shape.dims().len() == 3 && shape.dims()[0] == 3), "Incorrect format");

    let img_array = if shape.dims().len() == 3 {
        img_array.permute((1, 2, 0))?
    } else {
        img_array.clone()
    };

    let (height, width) = (shape.dims()[0], shape.dims()[1]);
    let data = img_array.to_dtype(DType::U8)?.to_vec2::<u8>()?;

    for row in data {
        for &pixel in row.iter() {
            let char = match pixel {
                0..=63 => ' ',
                64..=127 => '.',
                128..=191 => '*',
                192..=255 => '#',
                _ => '?',
            };
            print!("{}", char);
        }
        println!();
    }
    Ok(())
}