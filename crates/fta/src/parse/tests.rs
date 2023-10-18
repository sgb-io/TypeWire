#[cfg(test)]
mod tests {
    use crate::parse::parse_module;

    #[test]
    fn test_parse_module() {
        let ts_code = r#"
            function add(a: number, b: number): number {
                return a + b;
            }

            const myResult = add(23, 56);
            console.log(myResult); // 79
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 8, "Incorrect line count");
    }

    #[test]
    fn it_ignores_comments() {
        let ts_code = r#"
            /*
            This is a multi-line comment.
            You can write as many lines as you want.
            Each line will be part of the comment until the closing tag.
            */
            function add(a: number, b: number): number {
                return a + b;
            }
            
            // This is a single-line comment.
            const myResult = add(23, 56);
            console.log(myResult); // 79
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 8, "Incorrect line count");
    }
}
