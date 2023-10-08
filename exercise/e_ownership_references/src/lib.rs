pub fn inspect(args: &String) {
    let is_plural = if args.ends_with("s") { true } else { false };
    {
        if is_plural {
            print!("{} is plural!\n", args)
        } else {
            print!("{}\n", args)
        }
    }
}

pub fn change(arg: &mut String) {
    let is_plural = if arg.ends_with("s") { true } else { false };
    {
        if !is_plural {
            arg.push_str("s")
        }
    }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn bedazzle(arg: &mut String) {
    *arg = "sparkly".to_string()
}
