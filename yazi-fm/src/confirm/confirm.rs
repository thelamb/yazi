use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, text::{Line, Text}, widgets::{Block, BorderType, Borders, Paragraph, Widget}};
use yazi_config::THEME;

// use yazi_plugin::elements::Paragraph;
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
		let outer_layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
			.split(area);

		let inner_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
			.split(outer_layout[1]);

		Block::bordered()
			.border_type(BorderType::Rounded)
			.border_style(THEME.input.border)
			.title(Line::styled(&confirm.title(), THEME.input.title))
			.render(area, buf);

		Paragraph::new(confirm.message().split('\n').map(Line::from).collect::<Vec<Line>>())
			.render(outer_layout[0], buf);

		Paragraph::new("[Y]es").block(Block::bordered()).render(inner_layout[0], buf);
		Paragraph::new("(N)o").block(Block::bordered()).render(inner_layout[1], buf);
	}
}
