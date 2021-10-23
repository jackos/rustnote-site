# Trait Objects `dyn T`

These allow for multiple concrete types to fill in for an object at runtime.

Example below is a user adding UI elements at runtime, we don't know what they'll add at compile time so we keep a Vector, as long as it implements `Draw` it can be added to the list. Then we can loop over each item and call `draw()`


```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub trait Draw {
    fn draw(&self);
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a Button with width: {} height: {} label: {}",
            self.width, self.height, self.label
        );
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a Button with width: {} height: {} options: {:?}",
            self.width, self.height, self.options
        )
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
}
```

Must be object safe, rules are the all methods must:

- Not return Self.
- Have no generic type parameters.