use std::fs;

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

    let mut path_to_page = String::from("pages/page_");
    path_to_page.push_str(&*page_number.to_string());
    path_to_page.push_str(".txt");

    let content = fs::read_to_string(path_to_page).expect("Something went wrong reading the file.");

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
