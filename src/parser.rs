use std::fs;
use std::io;

pub struct Page {
    pub elements: Vec<PageElement>
}

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

pub fn parse_page(path_to_page: String) -> Result<Page, io::Error> {
    let mut page_elements: Vec<PageElement> = vec![];

    let content = fs::read_to_string(path_to_page)?;

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
    
    Ok(Page { elements: page_elements })
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
