#[derive(Debug)]
pub enum Color {
    Blue,
    Green,
}

pub struct MagicalBall {
    color: Color,
    secret_command: String,
}

impl MagicalBall {
    pub fn new() -> Self {
        MagicalBall {
            color: Color::Blue,
            secret_command: "whisper".to_string(),
        }
    }

    //private method. Only the owner can use it.
    fn change_color(&mut self, command: &str) -> bool {
        if command == self.secret_command {
            self.color = match self.color {
                Color::Blue => Color::Green,
                Color::Green => Color::Blue,
            };
            true
        } else {
            false
        }
    }

    pub fn prove_magic(&mut self) -> bool {
        //simulate using the secret command without revealing it.
        let secret_command_clone = self.secret_command.clone();

        self.change_color(&secret_command_clone)
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}

fn main() {
    let mut ball = MagicalBall::new();
    println!("Initial color: {:?}", ball.get_color());

    // Prove the magic without revealing the secret command
    if ball.prove_magic() {
        println!("After magic proof: {:?}", ball.get_color());
    } else {
        println!("Magic proof failed.");
    }
}
