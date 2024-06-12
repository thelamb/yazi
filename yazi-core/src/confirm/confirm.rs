use anyhow::Result;
use tokio::sync::oneshot::Sender;
use yazi_config::popup::Position; //, INPUT};

#[derive(Default)]
pub struct Confirm {
	pub(super) title:   String,
	pub(super) message: String,

	pub position: Position,

	pub(super) callback: Option<Sender<Result<bool>>>,

	pub visible: bool,
}

impl Confirm {
	#[inline]
	pub fn message(&self) -> String { self.message.clone() }

	#[inline]
	pub fn title(&self) -> String { self.title.clone() }
}
