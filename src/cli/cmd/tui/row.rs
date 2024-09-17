use crate::utils::format::Format;

#[derive(Debug)]
pub struct Row {
    pub data: Vec<u8>,
    pub offset: usize,
}

impl Row {
    pub fn parse(data: &[u8], offset: usize) -> Self {
        Self {
            data: data.to_vec(),
            offset,
        }
    }

    pub fn format_offset(&self) -> String {
        let res = Format::Octal.format(self.offset as u8);
        if res.len() > 8 {
            return format!("{:0>8}", res);
        }

        let mut padding = String::from("");
        for _ in 0..(8 - res.len()) {
            padding.push_str(&"·");
        }
        format!("{}{}", padding, res)
    }
}
