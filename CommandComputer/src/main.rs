use command_computer::{CommandLineComputer, UserType};

fn main() {
    let mut typer = UserType::new(CommandLineComputer);

    loop {
        typer.type_expr();
        println!("Result: {}", typer.compute());
    }
}
