use dialoguer::{theme::ColorfulTheme, Input};

pub fn input(question: &str) -> String {
    let input_value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .interact_text()
        .unwrap();
    return input_value;
}
