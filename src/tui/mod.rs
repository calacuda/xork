use crate::CommandEntered;
use crate::zones::Zone;
use bevy::prelude::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::Direction as TermUiDir,
    prelude::*,
    text::Line,
    widgets::Wrap,
    widgets::{Block, Borders, Paragraph},
};
use std::io::Stdout;
use std::num::NonZero;
use std::time::Duration;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

// pub mod chat_widget;

/// App holds the state of the application
#[derive(Resource)]
pub struct ClientTui {
    /// Current value of the input box
    input: Input,
    /// messages/responces from the game
    pub messages: Vec<String>,
    /// game anouncments in the order they were received
    pub anouncements: Vec<String>,
}

impl Default for ClientTui {
    fn default() -> Self {
        Self {
            input: Input::default(),
            messages: Vec::new(),
            anouncements: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct Term(pub Terminal<CrosstermBackend<Stdout>>);

/// a syntem to update the tui
pub fn tui_update(
    mut tui: ResMut<ClientTui>,
    mut terminal: ResMut<Term>,
    mut exit: EventWriter<AppExit>,
    mut cmd_events: EventWriter<CommandEntered>,
    zone: Res<Zone>,
) {
    let Ok(_) = terminal.0.draw(|f| ui(f, &tui, zone.to_owned())) else {
        exit.send(AppExit::Error(NonZero::new(5 as u8).unwrap()));
        return;
    };

    if event::poll(Duration::from_nanos(1)).is_ok_and(|b| b) {
        let Ok(ev) = event::read() else {
            exit.send(AppExit::Error(NonZero::new(6 as u8).unwrap()));
            return;
        };

        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Enter => {
                    let input = tui.input.value().to_lowercase();
                    if input == "quit" || input == "exit" || input == ":q" {
                        exit.send(AppExit::Success);
                    } else {
                        let mesg: String = tui.input.value().into();
                        // tui.messages.push(mesg.clone());
                        cmd_events.send(CommandEntered(mesg));

                        tui.input.reset();
                    }
                }
                _ => {
                    tui.input.handle_event(&Event::Key(key));
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &ClientTui, zone: Zone) {
    let outer_most_layout = Layout::default()
        .direction(TermUiDir::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.area());
    let right_layout = Layout::default()
        .direction(TermUiDir::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(4 * 4 + 2)])
        .split(outer_most_layout[1]);
    let left_layout = Layout::default()
        .direction(TermUiDir::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(outer_most_layout[0]);
    let left_top_layout = Layout::default()
        .direction(TermUiDir::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(left_layout[0]);
    let left_left_top_layout = Layout::default()
        .direction(TermUiDir::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(left_top_layout[0]);

    let input = {
        let left_width = left_layout[1].width.max(3) - 3;
        let scroll = app.input.visual_scroll(left_width as usize);
        let input = Paragraph::new(app.input.value())
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Command"));
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor_position((
            // Put cursor past the end of the input text
            left_layout[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            left_layout[1].y + 1,
        ));
        input
    };
    let player_stats = {
        // let width = right_layout[0].width.max(3) - 2;
        let stats = Paragraph::new("")
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Player"));
        stats
    };
    let party_stats = {
        // let width = right_layout[1].width.max(3) - 2;
        let stats = Paragraph::new("")
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Party"));
        stats
    };
    let chat = {
        // let items = merge_chat_and_sys(
        //     chat_messages.all_from_room(chat_room),
        //     app.anouncements.clone(),
        // );
        // let mut chat = ChatWidget::<ChatWidgetItem>::new(
        //     items,
        //     // chat_room,
        // );
        // chat.block = Some(Block::default().borders(Borders::ALL).title("Chat"));
        // chat
        let chat = Paragraph::new("")
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Chat"));
        chat
    };
    let main_block = {
        // let width = left_top_layout[1].width.max(3) - 2;
        Paragraph::new(
            [
                Line::from(zone.name),
                Line::from(""),
                Line::from(""),
                Line::from(zone.description),
                Line::from(""),
                // Line::from("upon closer inspection you notice:"),
                Line::from(zone.examine.unwrap_or(String::new())),
            ]
            .to_vec(),
        )
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("View"))
    };

    f.render_widget(player_stats, right_layout[0]);
    f.render_widget(party_stats, right_layout[1]);
    f.render_widget(&chat, left_left_top_layout[0]);
    // f.render_widget(&cmd_output, left_left_top_layout[1]);
    // main_messages
    //     .into_iter()
    //     .for_each(|message| f.render_widget(message, left_top_layout[1]));
    f.render_widget(main_block, left_top_layout[1]);

    f.render_widget(input, left_layout[1]);
}
