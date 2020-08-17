mod printer;
mod parser;

use printer::print_page;

use std::io::{stdout, Write};
use std::env;
use std::process;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    Result
};

fn main() -> Result<()> {
    clear_all()?;

    let mut page_number = 0;

    // The first arg is the program name, and the second is the page to display.
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        page_number = match args[1].parse::<usize>() {
            Err(_) => {
                printer::error_print("The page number must be a valid, positive integer.\n")?;
                process::exit(1);
            },
            Ok(value) => value
        };
    }

    print_page(&page_number)?;

    loop {
        match read()? {
            // Handles Control + c
            Event::Key(KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code,
            }) => {
                if code == KeyCode::Char('c') {
                    next_page(&mut page_number)?;
                }
            },

            // Handles Enter
            Event::Key(event) => {
                if event.code == KeyCode::Enter {
                    next_page(&mut page_number)?;
                }
            },
            _ => {}
        }
    }
}

fn clear_all() -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All)
    )?;

    Ok(())
}

fn next_page(page_number: &mut usize) -> Result<()> {
    clear_all()?;

    *page_number += 1;

    print_page(page_number)?;

    println!("\n\nPage {}", page_number);

    Ok(())
}
