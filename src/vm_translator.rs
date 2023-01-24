use {
    std::{fs::File, io::{BufReader, BufRead, Write}},
    lazy_static::lazy_static,
    regex::Regex,
};

lazy_static! {
    static ref REGEX_COMMENT: Regex = Regex::new("//.*").unwrap();
    static ref REGEX_WHITESPACES: Regex = Regex::new("(^\\s|\\s*$)").unwrap();
}

struct CodeWriter {
    output_file: File,
}

impl CodeWriter {
    pub fn new(output_file: File) -> Self {
        Self {
            output_file,
        }
    }

    fn write(&mut self, line: &str) {
        self.output_file.write(&line.as_bytes()).unwrap();
    }

    fn write_comment(&mut self, text: &str) {
        self.write(&format!("// {}", text));
    }

    fn write_push(&self, segment: &str, address: &str) {
        unimplemented!()
    }

    fn write_pop(&self, segment: &str, address: &str) {
        unimplemented!()
    }

    fn write_add(&self) {
        unimplemented!()
    }

    fn write_sub(&self) {
        unimplemented!()
    }

    fn write_eq(&self) {
        unimplemented!()
    }

    fn write_lt(&self) {
        unimplemented!()
    }

    fn write_gt(&self) {
        unimplemented!()
    }

    fn write_neg(&self) {
        unimplemented!()
    }

    fn write_and(&self) {
        unimplemented!()
    }

    fn write_or(&self) {
        unimplemented!()
    }

    fn write_not(&self) {
        unimplemented!()
    }
}

struct Parser {
    writer: CodeWriter,
}

impl Parser {
    pub fn new(writer: CodeWriter) -> Self {
        Self {
            writer,
        }
    }

    pub fn parse_file(&mut self, file: File) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = self.strip(&line.unwrap());

            if line.is_empty() {
                continue;
            }

            self.parse_line(&line);
        }
    }

    fn strip(&self, line: &str) -> String {
        let line = REGEX_COMMENT.replace(line, "");
        let line = REGEX_WHITESPACES.replace(&line, "");

        line.to_string()
    }

    fn parse_line(&mut self, line: &str) {
        self.writer.write_comment(line);
        
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let cmd = tokens[0];
        let args = &tokens[1..];

        match cmd {
            "push" => self.writer.write_push(args[0], args[1]),
            "pop" => self.writer.write_pop(args[0], args[1]),
            "add" => self.writer.write_add(),
            "sub" => self.writer.write_sub(),
            "eq" => self.writer.write_eq(),
            "lt" => self.writer.write_lt(),
            "gt" => self.writer.write_gt(),
            "neg" => self.writer.write_neg(),
            "and" => self.writer.write_and(),
            "or" => self.writer.write_or(),
            "not" => self.writer.write_not(),
            other => panic!("Unknown command {}", other),
        }
    }
}

pub fn translate_file(input_file: File, output_file: File) {
    let writer = CodeWriter::new(output_file);
    let mut parser = Parser::new(writer);
    parser.parse_file(input_file);
}