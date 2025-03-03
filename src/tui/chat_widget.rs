use crate::{
    client::chat::{ChatMessage, ChatMessages, ChatMessagesUtils},
    common::messages::ChatRoom,
};
use chrono::{DateTime, Local};
use ratatui::{
    prelude::*,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Debug)]
pub enum ChatWidgetItem {
    ChatMessage(ChatMessage),
    SystemMessage(String),
}

pub struct ChatWidget<'a, T> {
    pub messages: Vec<T>,
    pub block: Option<Block<'a>>,
}

impl ChatWidget<'_, ChatMessage> {
    pub fn new(msg: ChatMessages<'_, '_, '_>, room: ChatRoom) -> Self {
        Self {
            messages: msg.all_from_room(room),
            block: None,
        }
    }
}

impl ChatWidget<'_, ChatWidgetItem> {
    pub fn new(msg: Vec<ChatWidgetItem>) -> Self {
        Self {
            messages: msg,
            block: None,
        }
    }
}

impl Widget for &ChatWidget<'_, ChatWidgetItem> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let w = area.width;

        let mesgs: Vec<_> = self
            .messages
            .iter()
            .map(|m| {
                let message: Vec<_> = match m {
                    ChatWidgetItem::ChatMessage(m) => [
                        Line::from(
                            [
                                // chatters username
                                Span::raw(format!("{}: ", m.player)).style(Style::new().blue()),
                                // chat message time stamp
                                Span::raw(format!("{}", m.time_stamp)).style(Style::new().black()),
                            ]
                            .to_vec(),
                        ),
                        Line::from(m.message.clone()).style(Style::default()),
                    ]
                    .into(),
                    ChatWidgetItem::SystemMessage(s) => {
                        [Line::from(s.clone()).style(Style::new().black())].into()
                    }
                };
                // println!("{m:?}");

                message
            })
            .collect();

        let layout = Layout::vertical(
            (0..mesgs.len())
                .zip(mesgs.clone().into_iter())
                .map(|(_, m)| Constraint::Length(Text::from(m).to_string().len() as u16 / w + 2)),
        );

        let areas = if let Some(block) = &self.block {
            block.render(area, buf);
            let buf = self.block.inner_if_some(area);
            layout.split(buf)
        } else {
            layout.split(area)
        };

        mesgs.into_iter().enumerate().for_each(|(i, m)| {
            Paragraph::new(m)
                .wrap(Wrap { trim: true })
                .render(areas[i], buf)
        });
    }
}

impl Widget for &ChatWidget<'_, ChatMessage> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let w = area.width;

        let mesgs: Vec<_> = self
            .messages
            .iter()
            .map(|m| {
                let message: Vec<_> = [
                    Line::from(
                        [
                            // chatters username
                            Span::raw(format!("{}: ", m.player)).style(Style::new().blue()),
                            // chat message time stamp
                            Span::raw(format!("{}", m.time_stamp)).style(Style::new().black()),
                        ]
                        .to_vec(),
                    ),
                    Line::from(m.message.clone()).style(Style::default()),
                ]
                .into();
                // let l1 = Line::from(l1);
                // let message = Paragraph::new(message).wrap(Wrap { trim: true });
                // let messages: Vec<_> = [l1, l2].into();
                message
            })
            .collect();

        let layout = Layout::vertical(
            (0..mesgs.len())
                .zip(mesgs.clone().into_iter())
                .map(|(_, m)| Constraint::Length(Text::from(m).to_string().len() as u16 / w + 2)),
        );

        let areas = if let Some(block) = &self.block {
            block.render(area, buf);
            let buf = self.block.inner_if_some(area);
            layout.split(buf)
        } else {
            layout.split(area)
        };

        mesgs.into_iter().enumerate().for_each(|(i, m)| {
            Paragraph::new(m)
                .wrap(Wrap { trim: true })
                .render(areas[i], buf)
        });
    }
}

/// for testing
impl Widget for &ChatWidget<'_, String> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical((0..self.messages.len()).map(|_| Constraint::Min(2)));

        let areas = if let Some(block) = &self.block {
            block.render(area, buf);
            let buf = self.block.inner_if_some(area);
            layout.split(buf)
        } else {
            layout.split(area)
        };

        self.messages
            .iter()
            .map(|m| {
                let message: Vec<_> = [
                    Line::from(
                        [
                            // chatters username
                            Span::raw(format!("{}: ", "Server")).style(Style::new().blue()),
                            // chat message time stamp
                            Span::raw(format!("{}", "<TIME_STAMP>")).style(Style::new().black()),
                        ]
                        .to_vec(),
                    ),
                    Line::from(m.clone()).style(Style::default()),
                ]
                .into();
                // let l1 = Line::from(l1);
                let message = Paragraph::new(message).wrap(Wrap { trim: true });
                // let messages: Vec<_> = [l1, l2].into();
                message
            })
            .enumerate()
            .for_each(|(i, m)| m.render(areas[i], buf));
    }
}

pub fn merge_chat_and_sys(
    mesgs: Vec<ChatMessage>,
    sys_messages: Vec<(String, DateTime<Local>)>,
) -> Vec<ChatWidgetItem> {
    // let mesgs: Vec<_> = mesgs.into_iter().collect();
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::with_capacity(mesgs.len() + sys_messages.len() + 1);

    while mesgs.get(i).is_some() || sys_messages.get(j).is_some() {
        if let (Some(chat_msg), Some(sys_msg)) = (mesgs.get(i), sys_messages.get(j)) {
            if chat_msg.time_stamp >= sys_msg.1 {
                res.push(ChatWidgetItem::SystemMessage(sys_msg.0.clone()));
                j += 1;
            } else {
                res.push(ChatWidgetItem::ChatMessage(chat_msg.clone().clone()));
                i += 1;
            }
        } else if let Some(chat_msg) = mesgs.get(i)
            && sys_messages.get(j).is_none()
        {
            res.push(ChatWidgetItem::ChatMessage(chat_msg.clone().clone()));
            i += 1;
        } else if let Some(sys_msg) = sys_messages.get(j)
            && mesgs.get(i).is_none()
        {
            res.push(ChatWidgetItem::SystemMessage(sys_msg.0.clone()));
            j += 1;
        } else {
            // println!("breaking at {}", res.len());
            break;
        }
    }

    res
}
