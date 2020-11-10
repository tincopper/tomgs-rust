
#[cfg(test)]
mod tests {

  #[test]
  fn main1() {
    // 定义定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有的元素，如果初始化的数据相同，可以使用如下用法
    let ys: [i32; 500] = [0; 500];

    // 遍历
    // 在目前的标准库中，数组本身没有实现IntoIterator trait，但是数组切片是实现了的。所以我们可以直接在for in循环中使用数组切片，而不能直接使用数组本身。
    for x in &xs {
      println!("{}", x);
    }

    let guess: u32 = "42".parse().expect("Not a number!");
    let guess = "42".parse::<i32>().expect("Not a number!");

    println!("{}", guess);

  }

  #[test]
  fn main2() {
    fn mut_array(a: &mut [i32]) {
      a[2] = 5;
    }

    println!("size of &[i32; 3] : {:?}", std::mem::size_of::<&[i32; 3]>());
    // &[T]类型占用了两个指针大小的内存空间
    println!("size of &[i32] : {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1, 2, 3];

    {
      // &mut [i32; 3] 自动转换为&mut [i32]数组切片类型传入函数mut_array
      let s: &mut [i32; 3] = &mut v;
      mut_array(s);
    }

    println!("{:?}", v);

  }

  // 利用unsafe代码把这个胖指针内部的数据打印出来看看
  // 胖指针内部的数据既包含了指向源数组的地址，又包含了该切片的长度。
  fn raw_slice(arr: &[i32]) {
    unsafe {
      let (val1, val2): (usize, usize) = std::mem::transmute(arr);
      println!("Value in raw pointer:");
      println!("value1: {:x}", val1); // 源地址
      println!("value2: {:x}", val2); // 长度
    }
  }

  #[test]
  fn main3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let address: &[i32; 5] = &arr;
    println!("Address of arr: {:p}", address);
    raw_slice(address as &[i32]);
  }

}
