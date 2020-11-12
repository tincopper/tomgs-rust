use std::any::Any;

pub trait Component {
  // ...
}

pub trait ComponentAny: Component + Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

// 只要实现了Component意味着实现了ComponentAny
impl<T> ComponentAny for T
  where T: Component + Any
{
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

struct Demo {
  name: String,
}

impl Component for Demo {

}

struct Demo2 {
  name: String,
}

impl Component for Demo2 {

}

/*impl ComponentAny for Demo {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}*/

fn get_demos() -> Box<dyn ComponentAny> {
  Box::new(Demo {name: "1".to_string()})
}

#[test]
fn test_trait_translate() {
  let demo: Box<dyn ComponentAny> = get_demos();

  let any_demo = demo.as_any();
  let res = any_demo.downcast_ref::<Demo>();
  match res {
    Some(demo) => println!("{}", res.unwrap().name),
    None => println!("none"),
  }

  let res = any_demo.downcast_ref::<Demo2>();
  match res {
    Some(demo) => println!("{}", res.unwrap().name),
    None => println!("none"),
  }

}