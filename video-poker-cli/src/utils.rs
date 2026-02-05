use dialoguer::Select;

pub fn clear_screen() {
    print!("{esc}[2J{esc}[2;1H", esc = 27 as char);
}

pub fn press_any_to_continue() {
    Select::new().items([""]).default(0).interact().unwrap();
}
