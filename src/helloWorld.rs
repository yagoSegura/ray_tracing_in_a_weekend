


fn main(){
    let nx:i32 = 200;
    let ny:i32 = 100;

	println!("P3\n{} {}\n255",nx,ny);

    for j in (0..ny-1).rev(){
        for i in 0..nx-1{

            let r :f32 = (i as f32) / (nx as f32);
            let g :f32 = (j as f32) / (ny as f32);
            let b :f32 = 0.2;

            let ir : i32 = (255.99 * r) as i32;
            let ig : i32 = (255.99 * g) as i32;
            let ib : i32 = (255.99 * b) as i32;

            println!("{} {} {}",ir,ig,ib)
        }
    }
}
