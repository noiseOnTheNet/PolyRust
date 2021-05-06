use itertools::Itertools;
struct Poly{
  coeff : Vec<f32>
}
impl Poly{
  pub fn display(& self) -> String{
    self.coeff.iter()
    .enumerate()
    .map(|(i,c)| format!("{:e}*x^{}",c,i))
    .join(" + ")
  }
  pub fn eval(& self, x : f32) -> f32 {
    self.coeff.iter()
     .rev()
     .fold(0.0, |acc, c| x*acc + c )
  }
}
#[test]
fn test_display() {
  let x = Poly{coeff:vec![0.0,1.0,2.0]};
  assert_eq!("0e0*x^0 + 1e0*x^1 + 2e0*x^2",x.display());
}
#[test]
fn test_eval() {
  let x = Poly{coeff:vec![0.0,1.0,2.0]};
  assert_eq!(210.0,x.eval(10.0));
}

