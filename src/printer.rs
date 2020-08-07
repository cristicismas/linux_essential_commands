use super::parser;
use std::io::{stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result
};

pub fn print_page(page_number: &usize) -> Result<()> {
    let page_elements = parser::parse_page(page_number);

    for element in page_elements {
        if element.is_colored {
            accent_print(&element.text)?;
        } else {
            normal_print(&element.text)?;
        }
    }

    Ok(())
}

fn normal_print(text: &str) -> Result<()> {
    execute!(
        stdout(),
        Print(text)
    )?;

    Ok(())
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
