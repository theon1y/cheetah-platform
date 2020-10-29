use std::time::Instant;

use crate::udp::protocol::{DisconnectedStatus, FrameBuilder, FrameBuiltListener, FrameReceivedListener};
use crate::udp::protocol::commands::input::InCommandsCollector;
use crate::udp::protocol::commands::output::OutCommandsCollector;
use crate::udp::protocol::congestion::CongestionControl;
use crate::udp::protocol::disconnect::handler::DisconnectHandler;
use crate::udp::protocol::disconnect::watcher::DisconnectWatcher;
use crate::udp::protocol::frame::Frame;
use crate::udp::protocol::others::keep_alive::KeepAlive;
use crate::udp::protocol::others::rtt::RoundTripTimeImpl;
use crate::udp::protocol::reliable::ack::AckSender;
use crate::udp::protocol::reliable::replay_protection::FrameReplayProtection;
use crate::udp::protocol::reliable::retransmit::RetransmitterImpl;

///
/// Реализация игрового протокола, поверх ненадежного канала доставки данных (например, через UDP)
///
/// - логическая часть, без сети и сериализации
/// - надежная доставка
/// - защита от повторов
///
pub struct RelayProtocol {
	pub next_frame_id: u64,
	pub replay_protection: FrameReplayProtection,
	pub ack_sender: AckSender,
	pub retransmitter: RetransmitterImpl,
	pub disconnect_watcher: DisconnectWatcher,
	pub disconnect_handler: DisconnectHandler,
	pub in_commands_collector: InCommandsCollector,
	pub out_commands_collector: OutCommandsCollector,
	pub rtt: RoundTripTimeImpl,
	pub keep_alive: KeepAlive,
	pub additional_frame_builders: Vec<Box<dyn FrameBuilder>>,
	pub congestion_control: CongestionControl,
}

impl RelayProtocol {
	pub fn new() -> Self {
		Self {
			next_frame_id: 1,
			disconnect_watcher: Default::default(),
			replay_protection: Default::default(),
			ack_sender: Default::default(),
			in_commands_collector: Default::default(),
			out_commands_collector: Default::default(),
			retransmitter: Default::default(),
			additional_frame_builders: Default::default(),
			disconnect_handler: Default::default(),
			rtt: Default::default(),
			keep_alive: Default::default(),
			congestion_control: Default::default(),
		}
	}
	
	
	///
	/// Данный метод необходимо периодически вызывать
	/// для обработки внутренних данных
	/// 
	pub fn cycle(&mut self, now: &Instant) {
		self.congestion_control.rebalance(now, &self.rtt,  &mut self.retransmitter);
	}
	
	///
	/// Обработка входящего фрейма
	///
	pub fn on_frame_received(&mut self, frame: Frame, now: &Instant) {
		self.disconnect_watcher.on_frame_received(&frame, now);
		match self.replay_protection.set_and_check(&frame, now) {
			Ok(replayed) => {
				if !replayed {
					self.disconnect_handler.on_frame_received(&frame, now);
					self.ack_sender.on_frame_received(&frame, now);
					self.rtt.on_frame_received(&frame, now);
					self.in_commands_collector.on_frame_received(&frame, now);
				}
			}
			Err(_) => {}
		}
	}
	
	///
	/// Создание фрейма для отправки
	///
	pub fn build_next_frame(&mut self, now: &Instant) -> Option<Frame> {
		let mut builders: [&mut dyn FrameBuilder; 5] = [
			&mut self.ack_sender,
			&mut self.out_commands_collector,
			&mut self.disconnect_handler,
			&mut self.rtt,
			&mut self.keep_alive
		];
		let contains_data =
			builders
				.iter()
				.any(|h| h.contains_self_data(&now))
				||
				self.additional_frame_builders
					.iter()
					.any(|h| h.contains_self_data(&now));
		
		if contains_data {
			let mut frame = Frame::new(self.next_frame_id);
			self.next_frame_id += 1;
			builders
				.iter_mut()
				.for_each(|h| h.build_frame(&mut frame, now));
			
			self.additional_frame_builders
				.iter_mut()
				.for_each(|h| h.build_frame(&mut frame, now));
			
			self.retransmitter.on_frame_built(&frame, now);
			Option::Some(frame)
		} else {
			Option::None
		}
	}
	
	///
	/// Разорвана ли связь?
	///
	pub fn disconnected(&mut self, now: Instant) -> bool {
		self.retransmitter.disconnected(&now)
			|| self.disconnect_watcher.disconnected(&now)
			|| self.disconnect_handler.disconnected(&now)
	}
	
	pub fn add_frame_builder(&mut self, builder: Box<dyn FrameBuilder>) {
		self.additional_frame_builders.push(builder);
	}
	
	pub fn get_next_retransmit_frame(&mut self, now: &Instant) -> Option<Frame> {
		let next_frame_id = self.next_frame_id + 1;
		match self.retransmitter.get_retransmit_frame(&now, next_frame_id) {
			None => { Option::None }
			Some(frame) => {
				self.next_frame_id = next_frame_id;
				Option::Some(frame)
			}
		}
	}
}