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

    fn get_next_label(&mut self) -> String {
        unimplemented!()
    }

    fn write(&mut self, line: &str) {
        self.output_file.write(&format!("{}\n", line).as_bytes()).unwrap();
    }

    fn write_comment(&mut self, text: &str) {
        self.write(&format!("// {}", text));
    }

    fn pushd(&mut self) {
        self.write("@SP");
        self.write("M=M+1");
        self.write("A=M-A");
        self.write("M=D");
    }

    fn popd(&mut self) {
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
    }

    fn push_constant(&mut self, value: &str) {
        self.write(&format!("@{}", value));
        self.write("D=A");
        self.pushd();
    }

    fn push_register(&mut self, value: &str) {
        self.write(&format!("@{}", value));
        self.write("D=M");
        self.pushd();
    }

    fn push_segment(&mut self, segment_register: &str, address: &str) {
        self.write(&format!("@{}", segment_register));
        self.write("D=M");
        self.write(&format!("@{}", address));
        self.write("A=D+A");
        self.write("D=M");
        self.pushd();
    }

    fn write_push(&mut self, segment: &str, address: &str) {
        match segment {
            "constant" => self.push_constant(address),
            "pointer" => self.push_register(&format!("R{}", address.parse::<usize>().unwrap() + 3)),
            "temp" => self.push_register(&format!("R{}", address.parse::<usize>().unwrap() + 5)),
            "static" => self.push_register(&format!("S{}", address)),
            "local" => self.push_segment("LCL", address),
            "argument" => self.push_segment("ARG", address),
            "this" => self.push_segment("THIS", address),
            "that" => self.push_segment("THAT", address),
            other => panic!("Unknown push segment {}", other),
        }
    }

    fn pop_register(&mut self, register: &str) {
        self.popd();
        self.write(&format!("@{}", register));
        self.write("M=D");
    }

    fn pop_segment(&mut self, segment_register: &str, address: &str) {
        self.write(&format!("@{}", segment_register));
        self.write("D=M");
        self.write(&format!("@{}", address));
        self.write("D=D+A");
        self.write("@R13");
        self.write("M=D");
        self.popd();
        self.write("@R13");
        self.write("A=M");
        self.write("M=D");
    }

    fn write_pop(&mut self, segment: &str, address: &str) {
        match segment {
            "pointer" => self.pop_register(&format!("R{}", address.parse::<usize>().unwrap() + 3)),
            "temp" => self.pop_register(&format!("R{}", address.parse::<usize>().unwrap() + 5)),
            "static" => self.pop_register(&format!("S{}", address)),
            "local" => self.pop_segment("LCL", address),
            "argument" => self.pop_segment("ARG", address),
            "this" => self.pop_segment("THIS", address),
            "that" => self.pop_segment("THAT", address),
            other => panic!("Unknown pop segment {}", other),
        }
    }

    fn write_add(&mut self) {
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("A=A-1");
        self.write("M=D+M");
    }

    fn write_sub(&mut self) {
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("A=A-1");
        self.write("M=M-D");
    }

    fn write_eq(&mut self) {
        let label1 = self.get_next_label();
        let label2 = self.get_next_label();
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("A=A-1");
        self.write("D=M-D");
        self.write(&format!("@{}", label1));
        self.write("D;JEQ");
        self.write("@SP");
        self.write("A=M-1");
        self.write("M=0");
        self.write(&format!("@{}", label2));
        self.write("0;JMP");
        self.write(&format!("({})", label1));
        self.write("@SP");
        self.write("A=M-1");
        self.write("M=-1");
        self.write(&format!("({})", label2));
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