use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    if text.len() > width && width <= 3  {
        return ".".repeat(width);
    }
    if text.len() > width && width == 4 {
        return text.chars().take(1).collect::<String>() + "...";
    }
    
    if text.len() <= width {
        return text.to_string() + &" ".repeat(width - text.len());
    }
    let ellipsed = text.truncate_ellipse(3);
    ellipsed.to_string().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width: usize = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}