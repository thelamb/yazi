use ratatui::{buffer::Buffer, layout::Rect, text::{Line, Text}, widgets::{Block, BorderType, Paragraph, Widget}};
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

		let lines: Vec<_> = confirm.targets_list().iter().map(|t| Line::from(t.to_string())).collect();
		Paragraph::new(Text::from(lines))
			.block(
				Block::bordered()
					.border_type(BorderType::Rounded)
					.border_style(THEME.input.border)
					.title(Line::styled(&confirm.title(), THEME.input.title)),
			)
			.render(area, buf);
	}
}
