#[allow(dead_code, unused_variables)]
pub fn run() {
    struct Product {
        name: String,
        category: ProductCategory,
        price: f32,
        in_stock: bool,
    }

    enum ProductCategory {
        Books,
        Electronics,
        Clothing,
    }

    let category = ProductCategory::Books;

    let product = Product {
        category,
        name: String::from("Rust Programming"),
        price: 29.99,
        in_stock: true,
    };

    // # tuple
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let red = RGB(255, 0, 0);
    let color2 = CMYK(0, 100, 0, 0);

    enum Command {
        Undo,
        Redo,
        AddText(String),
        MoveCursor(i32, i32),
        Replace { from: String, to: String },
    }

    impl Command {
        fn serialize(&self) -> String {
            String::from("JSON string")
        }
    }

    let command = Command::Redo;
    let command = Command::AddText(String::from("Hello"));
    let command = Command::MoveCursor(32, 0);
    let command = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_str = command.serialize();
}
