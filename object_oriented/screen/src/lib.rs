


pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    // pub components: Vec<Box<dyn Draw>>,
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}



pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
        println!("draw button");
    }
}