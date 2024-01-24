use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude as Ra;

use crate::ui::popups::MsgPopup;
use crate::ui::EventState;

pub struct ConfirmPopup {
    msgpopup: MsgPopup,
    event_state: EventState,
}

impl ConfirmPopup {
    pub fn new() -> Self {
        Self {
            msgpopup: MsgPopup::new(),
            event_state: EventState::WorkDone,
        }
    }

    pub fn event(&mut self, ev: &Event) -> Result<EventState, ()> {
        if !self.msgpopup.is_visible() {
            return Ok(EventState::NotConsumed);
        }

        let mut event_state = EventState::NotConsumed;
        if let Event::Key(key) = ev {
            if key.kind != KeyEventKind::Press {
                return Ok(EventState::NotConsumed);
            }
            match key.code {
                KeyCode::Char('y') => {
                    self.msgpopup.hide();
                    return Ok(self.event_state.clone());
                }
                _ => {
                    event_state = self.msgpopup.event(ev).unwrap();
                }
            }
        }

        Ok(event_state)
    }

    pub fn draw(&mut self, f: &mut Ra::Frame, _area: Ra::Rect) {
        //! area is only used to keep the args
        self.msgpopup.draw(f, _area);
    }

    //pub fn is_visible(&self) -> bool {
    //    self.msgpopup.is_visible()
    //}
    //pub fn show(&mut self) {
    //    self.msgpopup.show()
    //}
    //pub fn hide(&mut self) {
    //    self.msgpopup.hide();
    //}

    pub fn popup_msg(&mut self, event_state: EventState, confirm_str: String) {
        self.event_state = event_state;
        self.msgpopup.push_txt_msg(confirm_str);
        self.msgpopup.show();
    }
}
