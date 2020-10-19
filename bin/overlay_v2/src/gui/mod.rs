mod state;
mod widgets;

use super::imgui::{self, Condition, StyleVar, Window};
use crate::{bridge::Avatar, requester::TextureRequester};
use protocol::message::client_dll_overlay::Message;
use state::GuiState;
use widgets::{bar::Bar, chars_panel::CharsPanel, chat::Chat, UiLogic};

const AVATARS_SIZES: [u16; 7] = [32, 48, 64, 80, 96, 112, 128];
const DEFAULT_AVATAR_SIZE_INDEX: usize = 2; // 64

type CharId = u32;

pub struct Gui {
    bar: Bar,
    chat: Chat,
    chars_panel: CharsPanel,
    avatars: Vec<Avatar>,
    pub(crate) state: GuiState,
    pub(crate) hide: bool,
    pub(crate) dirty: i8,
    //message_generator: MessageGenerator,
}
impl Gui {
    pub fn new() -> Self {
        Self {
            bar: Bar::new(),
            chat: Chat::new(),
            chars_panel: CharsPanel::new(),
            avatars: vec![],
            state: GuiState::new(),
            hide: false,
            dirty: 3,
            //message_generator: MessageGenerator::new(1),
        }
    }
    pub fn frame(&mut self, ui: &imgui::Ui, texture_requester: &mut TextureRequester) {
        if self.hide {
            self.dirty = 0;
            return;
        }

        /*if let Some(msg) = self.message_generator.message() {
            self.chat.push_message(msg);
        }*/
        if self.bar.show_faces {
            for avatar in &mut self.avatars {
                Self::render_window(ui, &mut self.state, avatar, texture_requester);
            }
        }

        Self::render_window(ui, &mut self.state, &mut self.bar, texture_requester);

        if self.bar.show_chars_panel {
            Self::render_window(
                ui,
                &mut self.state,
                &mut self.chars_panel,
                texture_requester,
            );
        }

        if self.bar.show_chat {
            Self::render_window(ui, &mut self.state, &mut self.chat, texture_requester);
        }

        let active = ui.is_mouse_dragging(imgui::MouseButton::Left);
        self.dirty = (self.dirty - 1).max(if active { 1 } else { 0 });
    }
    fn render_window<L: UiLogic>(
        ui: &imgui::Ui,
        state: &mut GuiState,
        logic: &mut L,
        texture_requester: &mut TextureRequester,
    ) {
        if !logic.visible(state) {
            return;
        }

        let title = logic.title();
        let window = Window::new(&title).title_bar(L::TITLE_BAR);
        let (window, size) = match logic.fixed_size(state) {
            Some(fixed) => {
                let size = [fixed.0 as f32, fixed.1 as f32];
                (
                    window
                        .size(size, Condition::Always)
                        .resizable(false)
                        .collapsible(false),
                    size,
                )
            }
            None => {
                let size = [L::INITIAL_SIZE.0 as f32, L::INITIAL_SIZE.1 as f32];
                (
                    window
                        .size(size, Condition::FirstUseEver)
                        .resizable(true)
                        .collapsible(true),
                    size,
                )
            }
        };
        let window = match (&state.game_rect, logic.fixed_position(state)) {
            (Some(rect), Some(fixed)) => {
                let pos = fixed.apply(rect, size);
                window.position(pos, Condition::Always).movable(false)
            }
            (_, None) => window.movable(true),
            _ => return,
        };
        let style = logic
            .padding(state)
            .map(|(x, y)| ui.push_style_var(StyleVar::WindowPadding([x as f32, y as f32])));
        window.scroll_bar(false).build(ui, || {
            logic.draw(ui, state, texture_requester);
        });

        if let Some(style) = style {
            style.pop(ui);
        }
    }
    // return true if avatars changed
    pub(crate) fn update_avatars(&mut self, avatars: Vec<Avatar>) -> bool {
        if self.avatars != avatars {
            self.avatars = avatars;
            self.state.update_avatars(&self.avatars);
            true
        } else {
            false
        }
    }
    pub(crate) fn push_message(&mut self, mut message: Message) {
        self.state.push_message(&mut message);
        self.chat.push_message(message);
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    use std::time::Instant;
    struct MessageGenerator {
        last_message_time: Instant,
        texti: usize,
        chari: usize,
        //sayi: usize,
        every_secs: u64,
    }

    use protocol::message::client_dll_overlay::Message;
    impl MessageGenerator {
        fn new(every_secs: u64) -> Self {
            Self {
                last_message_time: Instant::now(),
                texti: 0,
                chari: 0,
                every_secs,
            }
        }
        fn text(&mut self) -> String {
            let texts = ["foo", "bar", "baz", "foobar"];
            let ret = texts[self.texti % texts.len()].into();
            self.texti += 1;
            ret
        }
        fn char(&mut self) -> (CharId, Option<String>) {
            let chars = ["Anuri", "Frank", "VVish", "Sjaman"];
            let cr_id = self.chari % chars.len();
            self.chari += 1;
            (cr_id as CharId, Some(chars[cr_id].into()))
        }
        /*fn say_type(&mut self) -> String {
            let says = [];
            let ret = chars[self.chari % chars.len()].into();
            self.chari += 1;
            ret
        }*/
        fn message(&mut self) -> Option<Message> {
            if self.last_message_time.elapsed().as_secs() < self.every_secs {
                return None;
            }
            self.last_message_time = Instant::now();

            let (cr_id, name) = self.char();
            Some(Message {
                text: self.text(),
                say_type: fo_defines::Say::Normal,
                cr_id,
                delay: 0,
                name,
            })
        }
    }
}
