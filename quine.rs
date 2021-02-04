fn main() {
    let code = vec![
        "fn main() {",
        "    let code = vec![",
        "    ];",
        "    ",
        "    let mut i = 0;",
        "    ",
        "    for line in code.clone() {",
        "        if i >= 2 {",
        "            break;",
        "        }",
        "        ",
        "        println!(\"{}\", line);",
        "        i += 1;",
        "    }",
        "    ",
        "    for line in code.clone() {",
        "        println!(\"        {:?},\", line);",
        "    }",
        "    ",
        "    for (pos, line) in code.iter().enumerate() {",
        "        if pos == 0 || pos == 1 {",
        "            continue;",
        "        }",
        "        ",
        "        println!(\"{}\", line);",
        "    }",
        "}",
        "",
        "/* Author: Harry */",
    ];

    let mut i = 0;

    for line in code.clone() {
        if i >= 2 {
            break;
        }

        println!("{}", line);
        i += 1;
    }

    for line in code.clone() {
        println!("        {:?},", line);
    }

    for (pos, line) in code.iter().enumerate() {
        if pos == 0 || pos == 1 {
            continue;
        }

        println!("{}", line);
    }
}

/* Author: Harry */
