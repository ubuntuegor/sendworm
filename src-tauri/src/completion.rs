use magic_wormhole::Wordlist;
use std::sync::LazyLock;

static WORDLIST: LazyLock<Wordlist> = LazyLock::new(|| Wordlist::default_wordlist(2));

#[tauri::command]
pub fn get_completions(code: &str) -> Vec<String> {
    WORDLIST.get_completions(code).into_iter().take(4).collect()
}
