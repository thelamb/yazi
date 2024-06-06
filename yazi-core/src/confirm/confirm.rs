use anyhow::Result;
use tokio::sync::oneshot::Sender;
use yazi_config::popup::Position; //, INPUT};

#[derive(Default)]
pub struct Confirm {
	pub(super) title:   String,
	pub(super) targets: Vec<yazi_shared::fs::Url>,

	pub position: Position,

	pub(super) callback: Option<Sender<Result<bool>>>,

	pub visible: bool,
}

impl Confirm {
	#[inline]
	pub fn targets_list(&self) -> Vec<String> {
		self.targets.iter().map(|url| url.to_string()).collect()
	}

	#[inline]
	pub fn title(&self) -> String { self.title.clone() }
}
