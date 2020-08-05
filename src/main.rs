use std::io::{stdout, Write};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result
};

fn main() -> Result<()> {
    clear_all()?;
    accent_print("Hi!\n\n")?;

    println!("This is a small program made to help you learn some basic linux commands.\n");

    println!("You are in a terminal window right now. From this window, you can run very powerful commands that let you control your system (or other systems).\n");

    println!("This is by no means a complete linux mastery tutorial, but rather it is supposed to give you some direction and teach you to be self-sufficient enough to learn more stuff.\n");

    println!("I recommend opening another terminal window (usually done by pressing CTRL + ALT + T), to test new commands as soon as you learn them.\n");

    accent_print("To go to the next 'page', use ENTER.\n")?;

    println!("\n");

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Enter {
                    next_page();
                }
            },
            _ => {}
        }
    }
}

fn accent_print(text: &str) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(text),
        ResetColor
    )?;

    Ok(())
}

fn clear_all() -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All)
    );

    Ok(())
}

fn next_page() -> Result<()> {
    clear_all()?;

    Ok(())
}
