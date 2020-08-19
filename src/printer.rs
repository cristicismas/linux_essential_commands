use super::parser::Page;
use std::io::{stdout, stderr, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result
};

pub fn print_page(page: Page) -> Result<()> {
    for element in page.elements {
        if element.is_colored {
            accent_print(&element.text)?;
        } else {
            normal_print(&element.text)?;
        }
    }

    Ok(())
}

pub fn normal_print(text: &str) -> Result<()> {
    execute!(
        stdout(),
        Print(text)
    )?;

    Ok(())
}

pub fn accent_print(text: &str) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(text),
        ResetColor
    )?;

    Ok(())
}

pub fn error_print(text: &str) -> Result<()> {
    execute!(
        stderr(),
        SetForegroundColor(Color::Red),
        Print(text),
        ResetColor
    )?;

    Ok(())
}
