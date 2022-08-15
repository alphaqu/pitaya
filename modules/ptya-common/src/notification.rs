use std::sync::Arc;
use std::time::{Duration, Instant};
use anyways::audit::Audit;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct NotificationComp {
	notifications: Arc<RwLock<Vec<NotificationEntry>>>
}

impl NotificationComp {
	pub fn new() -> NotificationComp {
		NotificationComp {
			notifications: Default::default()
		}
	}

	pub async fn spawn(&self, notification: Notification) {
		self.notifications.write().await.push(NotificationEntry {
			value: notification,
			spawned: Instant::now(),
		});
	}
}

struct NotificationEntry {
	value: Notification,
	spawned: Instant,
}

pub struct Notification {
	pub title: String,
	pub description: String,
	pub severity: Severity,
	pub duration: Duration,
}

impl Notification {
	pub fn err(title: impl ToString, audit: Audit, severity: Severity) -> Notification {
		let mut buf = String::new();
		for (id, error) in audit.errors.into_iter().enumerate() {
			buf.push_str(&format!("{id}: {}", error.error.to_string()));
		}
		Notification {
			title: title.to_string(),
			description: buf.to_string(),
			duration: severity.get_duration(),
			severity,
		}
	}
}

#[derive(Copy, Clone)]
pub enum Severity {
	Info,
	Notice,
	Warning,
	Error,
	Critical,
	Emergency,
}

impl Severity {
	pub fn get_duration(&self) -> Duration {
		match self {
			Severity::Info => Duration::from_secs(5),
			Severity::Notice => Duration::from_secs(5),
			Severity::Warning => Duration::from_secs(10),
			Severity::Error => Duration::from_secs(30),
			Severity::Critical => Duration::from_secs(60),
			Severity::Emergency => Duration::MAX,
		}
	}
}