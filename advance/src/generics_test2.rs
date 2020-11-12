///  泛型参数约束
/// error[E0369]: binary operation `<` cannot be applied to type `T`
/// 也就是说类型T不能进行比较，需要实现PartialOrd的约束条件
fn max<T>(a: T, b: T) -> T
where
  T: PartialOrd
{
  if a < b {
    b
  } else {
    a
  }
}

#[test]
fn test_max() {
  let res = max(1, 2);
  println!("max is : {}", res);
}

/// 关联类型的泛型约束，使用where
///```
/// pub trait Iterator {
///     type Item;
///
///     fn max(self) -> Option<Self::Item>
///     where
///         Self: Sized,
///         Self::Item: Ord,
///     {
///         self.max_by(Ord::cmp)
///     }
/// }
///```
fn test() {

}