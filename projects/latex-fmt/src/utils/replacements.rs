pub fn replace_operator(s: &str) -> String {
    let r = match s {
        // Array
        "|->" => "\\mapsto",
        "<->" => "\\leftrightarrow",
        "<=>" => "\\Leftrightarrow",
        "==>" => "\\Longrightarrow",
        "->" => "\\rightarrow",
        "=>" => "\\Rightarrow",
        // Dots
        "..." => "\\cdots",
        // Arithmetic
        "+-" => "\\pm",
        "-+" => "\\mp",
        "==" => "\\equiv",
        "!=" => "\\ne",
        ">=" => "\\geqslant",
        "<=" => "\\leqslant",
        "<<" => "\\ll",
        ">>" => "\\gg",
        _ => s,
    };
    r.to_string()
}

pub fn replace_operator_name(s: &str) -> String {
    let names = ["arccot", "arcsec", "arccsc"];
    let f = &s[1..s.len()];
    if names.contains(&f) { format!("\\operatorname{{{}}}", f) } else { format!("\\{}", f) }
}
