
type Color = [u8;4];

use image::Rgba;
fn rgba2xyz(rgba: &[u8;4]) -> [f64;3] {

    let r = (rgba[0] as f64)/255.0;
    let g = (rgba[1] as f64)/255.0;
    let b = (rgba[2] as f64)/255.0; 


    let mut x = (r) * 0.4124 + (g) * 0.3576 + (b) * 0.1805;
    let mut y = (r) * 0.2126 + (g) * 0.7152 + (b) * 0.0722;
    let mut z = (r) * 0.0193 + (g) * 0.1192 + (b) * 0.9505;

    let scale_xyz = |v:f64| {
        if v>0.008856 {v.powf(1.0/3.0)}
        else {(7.787*v) + (16.0/116.0)}
    };
    x/=0.95047;
    x=scale_xyz(x);
    let l = if y>0.008856 {
        y = y.powf(1.0/3.0);
        116.0*y-16.0
    } else {
        let temp = y;
        y = y*7.787+(16.0/116.0);
        903.3*temp
    };
    z/=1.08833;
    z=scale_xyz(z);
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);
    [l,a,b]
}

fn dist(c1:&Color, c2:&Color) -> f64 {
    let l1 = rgba2xyz(c1);
    let l2 = rgba2xyz(c2);
    let v0 = (l1[0]-l2[0]).powi(2);
    let v1 = (l1[1]-l2[1]).powi(2);
    let v2 = (l1[2]-l2[2]).powi(2);
    v0+v1+v2
}
pub fn map_color(raw:Rgba<u8>, pallete: &Vec<[u8;4]>) -> (Rgba<u8>, usize) {
    let mut min_d = f64::MAX;
    let mut min_ind = pallete.len();
    for (ind, c) in pallete.iter().enumerate() {
        let d = dist(&raw.0, c);
        // dbg!(d);
        if d<min_d {
            min_d=d;
            min_ind = ind;
        }
    }
    (Rgba::from(pallete[min_ind]), min_ind)
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lab_dis() {
        let a:Color = [255, 168, 52, 255];
        let b:Color = [0xe1, 0xda, 0xc7, 0xff];
        dbg!(rgba2xyz(&a));
        dbg!(rgba2xyz(&b));
    }
}

