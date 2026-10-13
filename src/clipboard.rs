use std::io::Write;

use base64::{Engine as _, engine::general_purpose::STANDARD};

pub fn copy_to_clipboard(text: &str, clipboard: &mut Option<arboard::Clipboard>) -> bool {
    if let Some(cb) = clipboard {
        if cb.set_text(text.to_string()).is_ok() {
            return true;
        }
    }
    osc52_copy(text).is_ok()
}

fn osc52_copy(text: &str) -> std::io::Result<()> {
    let encoded = STANDARD.encode(text.as_bytes());
    let seq = format!("\x1b]52;c;{}\x07", encoded);
    let mut stdout = std::io::stdout();
    stdout.write_all(seq.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
