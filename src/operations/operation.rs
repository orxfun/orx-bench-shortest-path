use colored::Colorize;

pub trait Operation {
    fn name() -> &'static str;
    fn run();
    fn run_if_matches(command: &str) {
        if Self::name().to_lowercase() == command.to_lowercase() {
            let _oper = OperationRun::new(Self::name());
            Self::run();
        }
    }
}

struct OperationRun {
    operation_line: String,
}
impl OperationRun {
    pub fn new(name: &str) -> Self {
        let operation_line = Self::get_operation_line(name);
        println!("{}", operation_line.bold().magenta());
        Self { operation_line }
    }

    fn get_operation_line(name: &str) -> String {
        let side_len = if name.len() > HEADER_LEN {
            0
        } else {
            (HEADER_LEN - name.len()) / 2
        };
        let side = LINE_CHAR.repeat(side_len);
        format!("\n{} {} {}\n", side, name.to_uppercase(), side)
    }
}

impl Drop for OperationRun {
    fn drop(&mut self) {
        println!("{}\n", self.operation_line.bold().magenta().dimmed());
    }
}

const HEADER_LEN: usize = 120;
const LINE_CHAR: &str = "#";
