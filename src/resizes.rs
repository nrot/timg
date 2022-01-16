use image::{DynamicImage, imageops::FilterType};
pub fn resize(img: &DynamicImage, nw: u32, nh: u32, ft: FilterType)->DynamicImage{
    img.resize(nw, nh, ft)
}

pub fn resize_exact(img: &DynamicImage, nw: u32, nh: u32, ft: FilterType)->DynamicImage{
    img.resize_exact(nw, nh, ft)
}

pub fn resize_to_fill(img: &DynamicImage, nw: u32, nh: u32, ft: FilterType)->DynamicImage{
    img.resize_to_fill(nw, nh, ft)
}