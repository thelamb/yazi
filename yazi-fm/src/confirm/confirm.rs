use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, text::{Line, Text}, widgets::{Block, BorderType, Paragraph, Widget}};
use yazi_config::THEME;

use crate::Ctx;

pub(crate) struct Confirm<'a> {
	cx: &'a Ctx,
}

impl<'a> Confirm<'a> {
	pub(crate) fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl<'a> Widget for Confirm<'a> {
	fn render(self, _win: Rect, buf: &mut Buffer) {
		let confirm = &self.cx.confirm;
		let area = self.cx.area(&confirm.position);

		yazi_plugin::elements::Clear::default().render(area, buf);

		Paragraph::new(confirm.message().split('\n').map(Line::from).collect::<Vec<Line>>())
			.block(
				Block::bordered()
					.border_type(BorderType::Rounded)
					.border_style(THEME.input.border)
					.title(Line::styled(&confirm.title(), THEME.input.title)),
			)
			.render(chunks[0], buf);

		let text = Text::from(vec![
			Line::from("hello world 1"),
			Line::from("hello world 2"),
			Line::from("hello world 3"),
		])
		.centered();
		Paragraph::new(text).block(Block::bordered()).render(chunks[1], buf);
	}
}
