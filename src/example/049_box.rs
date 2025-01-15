trait UIComponent {
    fn render(&self) {
        println!("Rendering Something");
    }
}

#[allow(dead_code)]
struct Button {
    text: String,
}

impl UIComponent for Button {}

#[allow(dead_code)]
struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

#[allow(dead_code, unused_variables)]
pub fn run() {
    let button_a = Button {
        text: "A".to_string(),
    };
    let button_b = Box::new(Button {
        text: "B".to_string(),
    });
    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
