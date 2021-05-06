use itertools::Itertools;
struct Poly{
  coeff : Vec<f32>
}
fn main() {
  let x = Poly{coeff:vec![0.0,1.0,2.0]};
  let s :String = x.coeff.iter()
    .enumerate()
    .map(|(i,c)| format!("{:e}*x^{}",c,i))
    .join(" + ");
  println!("Hello World! {:?}",s);
}