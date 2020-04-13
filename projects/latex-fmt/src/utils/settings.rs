#[derive(Debug)]
pub struct Settings {
    pub arc_indent: usize,
    pub arc_symbol_set: String,
    pub arc_dict_separator: String,
    pub arc_list_separator: String,
    pub arc_list_max_length: usize,
    pub arc_end_newline: bool,
    pub arc_scope_blank_lines: usize,
    pub arc_number_separate: usize,
    pub arc_number_separate_lower: usize,
    pub arc_number_separate_complex: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            arc_symbol_set: String::from("="),
            arc_indent: 4,
            arc_dict_separator: String::from(""),
            arc_list_separator: String::from(""),
            arc_list_max_length: 128,
            arc_end_newline: true,
            arc_scope_blank_lines: 1,
            arc_number_separate: 0,
            arc_number_separate_lower: 0,
            arc_number_separate_complex: false,
        }
    }
}
