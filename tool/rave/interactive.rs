// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use itertools::Itertools;
use reedline::{
    ColumnarMenu, DefaultCompleter, Emacs, KeyCode, KeyModifiers, MenuBuilder, Reedline,
    ReedlineEvent, ReedlineMenu, Signal, default_emacs_keybindings,
};

pub fn choose(what: &str, options: &Vec<String>) -> Option<String> {
    let prompt = reedline::DefaultPrompt::new(
        reedline::DefaultPromptSegment::Basic(what.to_owned()),
        reedline::DefaultPromptSegment::Empty,
    );
    let mut completer = Box::new(DefaultCompleter::with_inclusions(&['_', '.']));

    completer.insert(options.clone());

    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    let mut keybindings = default_emacs_keybindings();

    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );
    let edit_mode = Box::new(Emacs::new(keybindings));
    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode);

    loop {
        let input = line_editor.read_line(&prompt);

        match input {
            Ok(Signal::Success(buffer)) => match buffer.as_str() {
                "all" => {
                    return Some(options.iter().join(","));
                }
                _ => {
                    return Some(buffer);
                }
            },
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) | Err(_) => return None,
        }
    }
}
