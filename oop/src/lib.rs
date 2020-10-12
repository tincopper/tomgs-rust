pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button.");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box.");
    }
}

// 不使用泛型
//pub struct Screen {
//    pub components: Vec<Box<dyn Draw>>,
//}

//impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// 使用泛型
pub struct Screen<T: Draw> {
  pub components: Vec<Box<T>>,
}

impl <T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Screen, Button, SelectBox, Draw};

    #[test]
    fn it_works() {
        let d1 = SelectBox {
            width: 12,
            height: 12,
            option: vec![String::from("Yew"),
                         String::from("No"),
                         String::from("MayBe"),]
        };
        let d2 = Button {
            width: 12,
            height: 12,
            label: String::from("OK"),
        };

        let screen = Screen {
            components: vec![
                Box::new(d1),
                //Box::new(d2),
            ],
        };

        screen.run();
    }
}
