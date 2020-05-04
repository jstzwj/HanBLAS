
use hanblas::*;

fn main() {
    {
        let mut a = 1f32;
        let mut b = 2f32;
        let mut c = 3f32;
        let mut s = 4f32;

        hanblas::srotg::srotg(&mut a, &mut b, &mut c, &mut s);
        println!("{}, {}, {}, {}", a,b,c,s);
    }
    
    {
        let mut sx:[f32;12] = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0];
        let ret = hanblas::sasum::sasum(3, &sx, 3);
        println!("{:?}", ret);
    }
}