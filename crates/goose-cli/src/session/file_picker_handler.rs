use rustyline::{Cmd, ConditionalEventHandler, Event, EventContext, RepeatCount, Movement};
use std::path::PathBuf;

use super::file_picker;

/// Custom event handler for file picker triggered by @ + Tab
pub struct FilePickerHandler;

impl FilePickerHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ConditionalEventHandler for FilePickerHandler {
    fn handle(
        &self,
        evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        ctx: &EventContext,
    ) -> Option<Cmd> {
        // Only handle Tab key events
        if let Event::KeySeq(key_events) = evt {
            if key_events.len() == 1 && key_events[0].0 == rustyline::KeyCode::Tab {
                let line = ctx.line();
                let pos = ctx.pos();
                
                // Find the last @ symbol in the line
                let at_char_pos = line.char_indices()
                    .filter(|(_, ch)| *ch == '@')
                    .map(|(idx, _)| idx)
                    .last();
                
                if let Some(at_pos) = at_char_pos {
                    // Make sure the @ is reasonably close to the cursor (within the current word)
                    let text_after_at = &line[at_pos + 1..pos];
                    if text_after_at.trim().is_empty() || !text_after_at.contains(' ') {
                        // Get current working directory
                        let current_dir = std::env::current_dir()
                            .unwrap_or_else(|_| PathBuf::from("."));
                        
                        // Show file picker
                        match file_picker::show_file_picker(&current_dir) {
                            Ok(Some(selected_file)) => {
                                // Create replacement text with @ prefix
                                let file_path = selected_file.to_string_lossy();
                                let replacement_text = format!("@{}", file_path);
                                
                                // Calculate the number of characters (not bytes) to move back
                                let chars_before_at = line[..at_pos].chars().count();
                                let chars_before_cursor = line[..pos].chars().count();
                                let chars_to_move_back = chars_before_cursor - chars_before_at;
                                
                                let movement = Movement::BackwardChar(chars_to_move_back);
                                
                                // Cmd::Replace should position cursor at end of replacement
                                return Some(Cmd::Replace(movement, Some(replacement_text)));
                            }
                            Ok(None) => {
                                // User cancelled, do nothing
                                return Some(Cmd::Noop);
                            }
                            Err(_) => {
                                // Error occurred, do nothing
                                return Some(Cmd::Noop);
                            }
                        }
                    }
                }
            }
        }
        
        // Not our event, let default handling proceed
        None
    }
} 