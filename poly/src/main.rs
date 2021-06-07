use itertools::Itertools;
use std::ops;
use std::vec::Vec;
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
  pub fn sum(& self, other : & Poly) -> Poly {
    let o_len = other.coeff.len();
    let s_len = self.coeff.len();
    let mut common_sum : Vec<f32> = self.coeff.iter()
                       .zip(other.coeff.iter())
                       .map(|(c1,c2)| c1 + c2)
                       .collect();
    if o_len > s_len {
      common_sum.extend_from_slice(& other.coeff[s_len..o_len])
    } else {
      common_sum.extend_from_slice(& self.coeff[o_len..s_len])
    };
    Poly{coeff : common_sum}
  }
  pub fn derive(& self) -> Poly {
    let derived_coeff : Vec <f32> = self.coeff.iter()
    .enumerate()
    .map(|(i,c)| (i as f32) *c)
    .collect();
    let stripped_coeff : Vec<f32> = derived_coeff[1..].iter().cloned().collect();
    Poly{ coeff: stripped_coeff }
  }
}

impl ops::Add<& Poly> for & Poly {
    type Output = Poly;

    fn add(self, rhs: & Poly) -> Poly {
      self.sum(rhs)
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
#[test]
fn test_sum(){
  let x = Poly{coeff:vec![0.0,1.0,2.0]};
  let y = Poly{coeff:vec![3.0,4.0,2.0,1.0]};
  let z = &x + &y;
  assert_eq!(z.coeff,vec![3.0,5.0,4.0,1.0]);
  let w = &y + &x;
  assert_eq!(w.coeff,vec![3.0,5.0,4.0,1.0]);
  let h = &x + &x;
  assert_eq!(h.coeff,vec![0.0,2.0,4.0]);
}
#[test]
fn test_derive(){
  let x = Poly{coeff:vec![3.0,4.0,1.0,2.0]};
  assert_eq!(x.derive().coeff,vec![4.0,2.0,6.0])
}

