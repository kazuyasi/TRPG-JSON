//! 状態遷移（純粋関数。I/O は行わない）

use super::action::Action;
use super::app::App;

/// Action を App に適用する
///
/// 副作用（クリップボード等）が必要になった段階で `Option<Effect>` を返すよう拡張する。
pub fn update(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,
        Action::SwitchTab(tab) => app.tab = tab,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::Tab;

    #[test]
    fn test_quit_sets_flag() {
        let mut app = App::default();
        update(&mut app, Action::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn test_switch_tab() {
        let mut app = App::default();
        assert_eq!(app.tab, Tab::Monster);
        update(&mut app, Action::SwitchTab(Tab::Spell));
        assert_eq!(app.tab, Tab::Spell);
        update(&mut app, Action::SwitchTab(Tab::Monster));
        assert_eq!(app.tab, Tab::Monster);
        assert!(!app.should_quit);
    }
}
