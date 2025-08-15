pub trait Color {
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn purple(&self) -> String;
    fn cyan(&self) -> String;
}

impl Color for str {
    fn red(&self) -> String {
        let mut text = String::from("\x1B[31m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
    fn green(&self) -> String {
        let mut text = String::from("\x1B[32m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
    fn yellow(&self) -> String {
        let mut text = String::from("\x1B[33m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
    fn blue(&self) -> String {
        let mut text = String::from("\x1B[34m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
    fn purple(&self) -> String {
        let mut text = String::from("\x1B[35m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
    fn cyan(&self) -> String {
        let mut text = String::from("\x1B[36m");
        text.push_str(self);
        text.push_str("\x1B[0m");
        text
    }
}

