

fn main() {
  trait T {
    fn method1(self: Self);
    fn method2(self: &Self);
    fn method3(self: &mut Self);
  }
  // 上下等价
  trait T2 {
    fn method1(self);
    fn method2(&self);
    fn method3(&mut self);
  }
}

