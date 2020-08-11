use std::fs;
use std::process;

#[derive(Debug, Clone, PartialEq)]
pub struct PageElement {
    pub is_colored: bool,
    pub text: String
}

impl PageElement {
    fn new(is_colored: bool, text: &str) -> PageElement {
        PageElement {
            is_colored,
            text: String::from(text)
        }
    }

    fn add_char(&mut self, character: char) {
        self.text.push(character);
    }
}

pub fn parse_page(page_number: &usize) -> Vec<PageElement> {
    let mut page_elements: Vec<PageElement> = vec![];

    let path_to_page = format!("pages/page_{}.txt", page_number);

    let content = match fs::read_to_string(path_to_page) {
        Ok(content) => content,
        Err(_) => {
            println!("Congrats! You've reached the end of this program.\n");
            println!("Now you are aware of some of the most used linux commands.\n");
            println!("You've still got a long way to go, but now that you have the basics down, it's a lot easier to get started with other commands.\n");
            process::exit(0);
        }
    };

    for line in content.lines() {
        // If the line contains "<", there is some accent in the page, in which case looping through the chars is necessary.
        // Otherwise, just push the line to the Vec.
        if line.contains("<") {
            let mut current_line_element = PageElement::new(false, "");

            for character in line.chars() {
                if character == '<' {
                    page_elements.push(current_line_element);
                    current_line_element = PageElement::new(true, "");
                } else if character == '>' {
                    page_elements.push(current_line_element);
                    current_line_element = PageElement::new(false, "");
                } else {
                    current_line_element.add_char(character);
                }
            }
            
            if current_line_element.text.len() > 0 {
                page_elements.push(current_line_element);
            }
        } else {
            let mut element = PageElement::new(false, line);

            if line.is_empty() {
                element.text = String::from("\n\n");
                page_elements.push(element);
            } else {
                page_elements.push(element);
            }
        }
    }
    
    return page_elements;
}

#[cfg(test)]
mod tests {
    use super::PageElement;

    #[test]
    fn test_add_char_impl() {
        let mut element = PageElement::new(false, "ABC");
        element.add_char('D');

        let correct_element = PageElement::new(false, "ABCD");

        assert_eq!(element, correct_element);
    }
}
