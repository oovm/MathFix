pub fn replace_operator(s: &str) -> String {
    let r = match s {
        // Array
        "|->" => "mapsto",
        "<->" => "leftrightarrow",
        "<=>" => "Leftrightarrow",
        "==>" => "Longrightarrow",
        "->" => "rightarrow",
        "=>" => "Rightarrow",
        // Dots
        "..." => "cdots",
        // Arithmetic
        "+-" => "pm",
        "-+" => "mp",
        "==" => "equiv",
        "!=" => "ne",
        ">=" => "geqslant",
        "<=" => "leqslant",
        "<<" => "ll",
        ">>" => "gg",
        _ => return format!("{}", s),
    };
    format!(" \\{} ", r)
}

pub fn replace_operator_name(s: &str) -> String {
    let names = ["arccot", "arcsec", "arccsc"];
    let symbol = &s[1..s.len()];
    if names.contains(&symbol) { format!("\\operatorname{{{}}}", symbol) } else { format!("\\{}", symbol) }
}
