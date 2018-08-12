fn main(){
        let nx:i32 = 200;
        let ny:i32 = 100;

	println!("P3\n{} {}\n255\n",nx,ny);
        for _ny_param in (0..ny-1).rev(){
            for _nx_param in 0..nx-1{
                println!("#")
            }   
        }
}
