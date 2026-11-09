use zellij_tile::prelude::{actions::Action, InputMode};

pub fn modes_to_string(mode: InputMode) -> String {
    match mode {
        InputMode::Normal => "Normal".to_string(),
        InputMode::Locked => "Locked".to_string(),
        InputMode::Resize => "Resize".to_string(),
        InputMode::Pane => "Pane".to_string(),
        InputMode::Tab => "Tab".to_string(),
        InputMode::Scroll => "Scroll".to_string(),
        InputMode::EnterSearch => "Enter Search".to_string(),
        InputMode::Search => "Search".to_string(),
        InputMode::RenameTab => "RenameTab".to_string(),
        InputMode::RenamePane => "RenamePane".to_string(),
        InputMode::Session => "Session".to_string(),
        InputMode::Move => "Move".to_string(),
        InputMode::Prompt => "Prompt".to_string(),
        InputMode::Tmux => "Tmux".to_string(),
    }
}

//  Zellij Version 0.44
pub fn actions_to_string(actions: Vec<Action>) -> String {
    actions
        .iter()
        .map(|action| match action {
            Action::Quit => "Quit".to_string(),
            Action::Write {
                key_with_modifier: _,
                bytes: _,
                is_kitty_keyboard_protocol: _,
            } => "Write".to_string(),
            Action::WriteChars { chars: _ } => "WriteChars".to_string(),
            Action::WriteToPaneId { bytes: _, pane_id } => format!("WriteToPaneId({:?})", pane_id),
            Action::WriteCharsToPaneId { chars: _, pane_id } => {
                format!("WriteCharsToPaneId({:?})", pane_id)
            }
            Action::Paste { chars: _, pane_id } => format!("Paste({:?})", pane_id),
            Action::SwitchToMode { input_mode } => format!("SwitchToMode({:?})", input_mode),
            Action::SwitchModeForAllClients { input_mode } => {
                format!("SwitchModeForAllClients({:?})", input_mode)
            }
            Action::Resize { resize, direction } => {
                format!("Resize({:?}, {:?})", resize, direction)
            }
            Action::FocusNextPane => "FocusNextPane".to_string(),
            Action::FocusPreviousPane => "FocusPreviousPane".to_string(),
            Action::SwitchFocus => "SwitchFocus".to_string(),
            Action::MoveFocus { direction } => format!("MoveFocus({:?})", direction),
            Action::MoveFocusOrTab { direction } => format!("MoveFocusOrTab({:?})", direction),
            Action::MovePane { direction } => format!("MovePane({:?})", direction),
            Action::MovePaneBackwards => "MovePaneBackwards".to_string(),
            Action::ClearScreen => "ClearScreen".to_string(),
            Action::DumpScreen { .. } => "DumpScreen".to_string(),
            Action::DumpLayout => "DumpLayout".to_string(),
            Action::SaveSession => "SaveSession".to_string(),
            Action::EditScrollback { ansi: _ } => "EditScrollback".to_string(),
            Action::ScrollUp => "ScrollUp".to_string(),
            Action::ScrollUpAt { position } => format!("ScrollUpAt({:?})", position),
            Action::ScrollDown => "ScrollDown".to_string(),
            Action::ScrollDownAt { position } => format!("ScrollDownAt({:?})", position),
            Action::ScrollToBottom => "ScrollToBottom".to_string(),
            Action::ScrollToTop => "ScrollToTop".to_string(),
            Action::PageScrollUp => "PageScrollUp".to_string(),
            Action::PageScrollDown => "PageScrollDown".to_string(),
            Action::HalfPageScrollUp => "HalfPageScrollUp".to_string(),
            Action::HalfPageScrollDown => "HalfPageScrollDown".to_string(),
            Action::ToggleFocusFullscreen => "ToggleFocusFullscreen".to_string(),
            Action::TogglePaneFrames => "TogglePaneFrames".to_string(),
            Action::ToggleActiveSyncTab => "ToggleActiveSyncTab".to_string(),
            Action::NewPane {
                direction,
                pane_name,
                start_suppressed,
            } => format!(
                "NewPane({:?}, {:?}, {})",
                direction, pane_name, start_suppressed
            ),
            Action::NewBlockingPane { .. } => "NewBlockingPane".to_string(),
            Action::EditFile { .. } => "EditFile".to_string(),
            Action::NewFloatingPane { .. } => "NewFloatingPane".to_string(),
            Action::NewTiledPane { .. } => "NewTiledPane".to_string(),
            Action::NewInPlacePane { .. } => "NewInPlacePane".to_string(),
            Action::NewStackedPane { .. } => "NewStackedPane".to_string(),
            Action::TogglePaneEmbedOrFloating => "TogglePaneEmbedOrFloating".to_string(),
            Action::ToggleFloatingPanes => "ToggleFloatingPanes".to_string(),
            Action::ShowFloatingPanes { .. } => "ShowFloatingPanes".to_string(),
            Action::HideFloatingPanes { .. } => "HideFloatingPanes".to_string(),
            Action::AreFloatingPanesVisible { .. } => "AreFloatingPanesVisible".to_string(),
            Action::CloseFocus => "CloseFocus".to_string(),
            Action::PaneNameInput { input } => {
                format!("PaneNameInput({})", String::from_utf8_lossy(input))
            }
            Action::UndoRenamePane => "UndoRenamePane".to_string(),
            Action::NewTab { .. } => "NewTab".to_string(),
            Action::NoOp => "NoOp".to_string(),
            Action::GoToNextTab => "GoToNextTab".to_string(),
            Action::GoToPreviousTab => "GoToPreviousTab".to_string(),
            Action::CloseTab => "CloseTab".to_string(),
            Action::GoToTab { index } => format!("GoToTab({})", index),
            Action::GoToTabName { name, create } => format!("GoToTabName({}, {})", name, create),
            Action::ToggleTab => "ToggleTab".to_string(),
            Action::TabNameInput { input } => {
                format!("TabNameInput({})", String::from_utf8_lossy(input))
            }
            Action::UndoRenameTab => "UndoRenameTab".to_string(),
            Action::MoveTab { direction } => format!("MoveTab({:?})", direction),
            Action::Run {
                command,
                near_current_pane: _,
                no_focus: _,
            } => format!("Run({:?})", command),
            Action::SetPaneColor { .. } => "SetPaneColor".to_string(),
            Action::Detach => "Detach".to_string(),
            Action::SetDarkTheme => "SetDarkTheme".to_string(),
            Action::SetLightTheme => "SetLightTheme".to_string(),
            Action::ToggleTheme => "ToggleTheme".to_string(),
            Action::SwitchSession { name, .. } => format!("SwitchSession({})", name),
            Action::LaunchOrFocusPlugin { plugin, .. } => {
                format!("LaunchOrFocusPlugin({})", plugin.location_string())
            }
            Action::LaunchPlugin { plugin, .. } => {
                format!("LaunchPlugin({})", plugin.location_string())
            }
            Action::MouseEvent { event } => format!("MouseEvent({:?})", event),
            Action::Copy => "Copy".to_string(),
            Action::Confirm => "Confirm".to_string(),
            Action::Deny => "Deny".to_string(),
            Action::SkipConfirm { .. } => "SkipConfirm".to_string(),
            Action::SearchInput { input } => {
                format!("SearchInput({})", String::from_utf8_lossy(input))
            }
            Action::Search { direction } => format!("Search({:?})", direction),
            Action::SearchToggleOption { option } => format!("SearchToggleOption({:?})", option),
            Action::ToggleMouseMode => "ToggleMouseMode".to_string(),
            Action::PreviousSwapLayout => "PreviousSwapLayout".to_string(),
            Action::NextSwapLayout => "NextSwapLayout".to_string(),
            Action::OverrideLayout { .. } => "OverrideLayout".to_string(),
            Action::QueryTabNames => "QueryTabNames".to_string(),
            Action::NewTiledPluginPane { plugin, .. } => {
                format!("NewTiledPluginPane({})", plugin.location_string())
            }
            Action::NewFloatingPluginPane { plugin, .. } => {
                format!("NewFloatingPluginPane({})", plugin.location_string())
            }
            Action::NewInPlacePluginPane { plugin, .. } => {
                format!("NewInPlacePluginPane({})", plugin.location_string())
            }
            Action::StartOrReloadPlugin { plugin } => {
                format!("StartOrReloadPlugin({})", plugin.location_string())
            }
            Action::CloseTerminalPane { pane_id } => format!("CloseTerminalPane({})", pane_id),
            Action::ClosePluginPane { pane_id } => format!("ClosePluginPane({})", pane_id),
            Action::FocusTerminalPaneWithId {
                pane_id,
                should_float_if_hidden: _,
                should_be_in_place_if_hidden: _,
            } => format!("FocusTerminalPaneWithId({})", pane_id),
            Action::FocusPluginPaneWithId {
                pane_id,
                should_float_if_hidden: _,
                should_be_in_place_if_hidden: _,
            } => format!("FocusPluginPaneWithId({})", pane_id),
            Action::RenameTerminalPane { pane_id, name } => format!(
                "RenameTerminalPane({}, {})",
                pane_id,
                String::from_utf8_lossy(name)
            ),
            Action::RenamePluginPane { pane_id, name } => format!(
                "RenamePluginPane({}, {})",
                pane_id,
                String::from_utf8_lossy(name)
            ),
            Action::RenameTab { tab_index, name } => {
                format!(
                    "RenameTab({}, {})",
                    tab_index,
                    String::from_utf8_lossy(name)
                )
            }
            Action::GoToTabById { id } => format!("GoToTabById({})", id),
            Action::CloseTabById { id } => format!("CloseTabById({})", id),
            Action::RenameTabById { id, name } => format!("RenameTabById({}, {})", id, name),
            Action::BreakPane => "BreakPane".to_string(),
            Action::BreakPaneRight => "BreakPaneRight".to_string(),
            Action::BreakPaneLeft => "BreakPaneLeft".to_string(),
            Action::RenameSession { name } => format!("RenameSession({})", name),
            Action::CliPipe { .. } => "CliPipe".to_string(),
            Action::KeybindPipe { .. } => "KeybindPipe".to_string(),
            Action::ListClients => "ListClients".to_string(),
            Action::ListPanes { .. } => "ListPanes".to_string(),
            Action::ListTabs { .. } => "ListTabs".to_string(),
            Action::CurrentTabInfo { .. } => "CurrentTabInfo".to_string(),
            Action::TogglePanePinned => "TogglePanePinned".to_string(),
            Action::StackPanes { .. } => "StackPanes".to_string(),
            Action::ChangeFloatingPaneCoordinates { .. } => {
                "ChangeFloatingPaneCoordinates".to_string()
            }
            Action::TogglePaneBorderless { .. } => "TogglePaneBorderless".to_string(),
            Action::SetPaneBorderless { .. } => "SetPaneBorderless".to_string(),
            Action::TogglePaneInGroup => "TogglePaneInGroup".to_string(),
            Action::ToggleGroupMarking => "ToggleGroupMarking".to_string(),
            Action::ScrollUpByPaneId { .. } => "ScrollUpByPaneId".to_string(),
            Action::ScrollDownByPaneId { .. } => "ScrollDownByPaneId".to_string(),
            Action::ScrollToTopByPaneId { .. } => "ScrollToTopByPaneId".to_string(),
            Action::ScrollToBottomByPaneId { .. } => "ScrollToBottomByPaneId".to_string(),
            Action::PageScrollUpByPaneId { .. } => "PageScrollUpByPaneId".to_string(),
            Action::PageScrollDownByPaneId { .. } => "PageScrollDownByPaneId".to_string(),
            Action::HalfPageScrollUpByPaneId { .. } => "HalfPageScrollUpByPaneId".to_string(),
            Action::HalfPageScrollDownByPaneId { .. } => "HalfPageScrollDownByPaneId".to_string(),
            Action::ResizeByPaneId { .. } => "ResizeByPaneId".to_string(),
            Action::MovePaneByPaneId { .. } => "MovePaneByPaneId".to_string(),
            Action::MovePaneBackwardsByPaneId { .. } => "MovePaneBackwardsByPaneId".to_string(),
            Action::ClearScreenByPaneId { .. } => "ClearScreenByPaneId".to_string(),
            Action::EditScrollbackByPaneId { .. } => "EditScrollbackByPaneId".to_string(),
            Action::ToggleFocusFullscreenByPaneId { .. } => {
                "ToggleFocusFullscreenByPaneId".to_string()
            }
            Action::TogglePaneEmbedOrFloatingByPaneId { .. } => {
                "TogglePaneEmbedOrFloatingByPaneId".to_string()
            }
            Action::CloseFocusByPaneId { .. } => "CloseFocusByPaneId".to_string(),
            Action::RenamePaneByPaneId { .. } => "RenamePaneByPaneId".to_string(),
            Action::UndoRenamePaneByPaneId { .. } => "UndoRenamePaneByPaneId".to_string(),
            Action::TogglePanePinnedByPaneId { .. } => "TogglePanePinnedByPaneId".to_string(),
            Action::FocusPaneByPaneId { .. } => "FocusPaneByPaneId".to_string(),
            Action::UndoRenameTabByTabId { .. } => "UndoRenameTabByTabId".to_string(),
            Action::ToggleActiveSyncTabByTabId { .. } => "ToggleActiveSyncTabByTabId".to_string(),
            Action::ToggleFloatingPanesByTabId { .. } => "ToggleFloatingPanesByTabId".to_string(),
            Action::PreviousSwapLayoutByTabId { .. } => "PreviousSwapLayoutByTabId".to_string(),
            Action::NextSwapLayoutByTabId { .. } => "NextSwapLayoutByTabId".to_string(),
            Action::MoveTabByTabId { .. } => "MoveTabByTabId".to_string(),
            // Tile 0.45.1 keeps adding Action variants; the match no longer
            // stays exhaustively in sync with upstream ports.
            _ => "Unknown Action".to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ")
}
