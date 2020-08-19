mod printer;
mod parser;

use printer::print_page;

use std::io::{stdout, Write, Error};
use std::env;
use std::process;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers}
};

fn main() -> crossterm::Result<()> {
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

    parse_and_print_page(&page_number)?;

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

fn clear_all() -> crossterm::Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All)
    )?;

    Ok(())
}

fn next_page(page_number: &mut usize) -> crossterm::Result<()> {
    clear_all()?;

    *page_number += 1;
    
    parse_and_print_page(page_number)?;

    println!("\n\nPage {}", page_number);

    Ok(())
}

fn parse_and_print_page(page_number: &usize) -> crossterm::Result<()> {
    let path_to_page = format!("pages/page_{}.txt", page_number);

    let page = match parser::parse_page(path_to_page) {
        Ok(page) => page,
        Err(_) => {
            let last_page = parser::parse_page(
                String::from("pages/end_of_program.txt")
            ).unwrap();

            print_page(last_page).unwrap();
        
            process::exit(1);
        }
    };

    print_page(page)?;

    Ok(())
}
