use std::io::{stdout, Write};

use termion::cursor;
use termion::event::Key;
use termion::event::{Event, MouseEvent};

mod command;
mod data;
mod tgui;
mod widgets;

use tgui::widget::WidgetContainer;
use tgui::{Container, TerminalBuilder};
use widgets::{Footer, FooterColorConfig, Header, HeaderColorConfig};

struct ColorConfig {
    header: &'static HeaderColorConfig,
    footer: &'static FooterColorConfig,
}

struct State {
    color_config: ColorConfig,
    header_text: String,
    input_text: String,
}

fn main() {
    let state = State {
        color_config: ColorConfig {
            header: &HeaderColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
            footer: &FooterColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
        },
        header_text: String::from("hello world"),
        input_text: String::from(""),
    };

    let components_fn = |container: &Container| {
        let header = Header {
            container: &container,
            color_config: state.color_config.header,
            display_text: &state.header_text,
        };
        let footer = Footer {
            container: &container,
            color_config: state.color_config.footer,
            input_text: &state.input_text,
        };

        let mut widget_container = WidgetContainer::default();
        widget_container.register(Box::new(&header));
        widget_container.register(Box::new(&footer));

        widget_container
    };

    let terminal = TerminalBuilder::new()
        .register_components(&components_fn)
        .draw(&|event: Event| {
            // let mut inputted_command = None;

            // match event {
            //     Event::Key(key) => match key {
            //         Key::Ctrl('c') => std::process::exit(0),
            //         Key::Char('\n') => {
            //             inputted_command = command::parse_input(&state.input_text);
            //             state.input_text.clear();
            //         }
            //         Key::Delete | Key::Backspace => {
            //             state.input_text.pop();
            //         }
            //         Key::Char(key) => {
            //             state.input_text.push(key);
            //         }
            //         _ => {}
            //     },
            //     // Event::Mouse(me) => write!(stdout, "{}", cursor::Goto(0, 0)).unwrap(),
            //     // Event::Unsupported(_) => {}
            //     _ => {}
            // }

            // match inputted_command {
            //     Some(command::CommandKind::Quit) => std::process::exit(0),
            //     _ => {}
            // }
        })
        .execute();

    terminal.run();

    // write!(stdout, "{}", termion::cursor::Show).unwrap();
}
