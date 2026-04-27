use ratatui_ex::app::{App, CurrentScene, InputMode, SceneEvent};
// use ratatui_ex::downloader::DownloadEvent;
use ratatui_ex::audio::{Player, Playlist, Song};
use ratatui_ex::ui::ui;
use flexi_logger::{FileSpec, Logger};
use ratatui::Terminal;
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::widgets::ListState;
use std::error::Error;
use std::io;
use std::time::Duration;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // if let Some(downloader) = &mut app.downloader
        //     && let Ok(event) = downloader.rx.try_recv()
        // {
        //     log::debug!("Got event {:?}", event);

        //     match event {
        //         DownloadEvent::Succesful(song_path) => {
        //             if let Some(player) = &mut app.player {
        //                 if let Some(added_song) = Song::new_from_path(song_path.into()) {
        //                     log::debug!("Added Song {:?}", added_song);
        //                     let song = Rc::new(added_song);
        //                     if player.playlist.borrow().name
        //                             != app.get_playlists().first().unwrap().borrow().name
        //                     {
        //                         // first is allways the all songs, the name should be unique
        //                         app.add_song_to_playlist(0, song.clone()); // Add to all songs
        //                     }
        //                     player.playlist.borrow_mut().add_song(song);
        //                 } else {
        //                     log::debug!("Couldn't add song");
        //                     app.error_message = Some("Couldn't add downloaded song".to_string());
        //                 }
        //             }
        //         }
        //         DownloadEvent::Failed(msg) => {
        //             app.error_message = Some(msg);
        //         }
        //     }
        // }
        // --snip--
        if event::poll(Duration::from_millis(100))? {
            // handle user input
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                } else {
                    app.error_message = None
                }

                match key.code {
                    KeyCode::Backspace => {
                        if let Some(input) = &mut app.input_mode {
                            match input {
                                InputMode::CreatePlaylist { playlist_name } => {
                                    playlist_name.pop();
                                }
                                InputMode::AddToPlaylist { .. }
                                | InputMode::SelectPlaylist {
                                    selected_playlist: _,
                                } => {}
                            }
                        }
                    }
                    KeyCode::Char(chr) => {
                        if let Some(input) = &mut app.input_mode {
                            match input {
                                InputMode::CreatePlaylist { playlist_name } => {
                                    playlist_name.push(chr);
                                }
                                InputMode::AddToPlaylist { .. }
                                | InputMode::SelectPlaylist {
                                    selected_playlist: _,
                                } => {}
                            }
                        }
                        match chr {
                            'q' => match app.curr_scene {
                                CurrentScene::Main => {
                                    app.curr_scene = CurrentScene::Exiting;
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            ' ' => match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        match (player.is_playing(), player.has_different_selected())
                                        {
                                            (true, true) | (false, true) | (false, false) => {
                                                match player.play() {
                                                    Ok(_) => {}
                                                    Err(_) => {
                                                        app.error_message = Some("Failed to play currently selected song due to decoder issue".to_string());
                                                    }
                                                }
                                            }
                                            (true, false) => player.pause(),
                                        }
                                    }
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            'a' => match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        match player.add_to_queue() {
                                            Ok(_) => {}
                                            Err(_) => {
                                                app.error_message =
                                                    Some("Failed to queue song".to_string())
                                            }
                                        }
                                    }
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            'p' => match app.curr_scene {
                                CurrentScene::Main => {
                                    app.curr_scene = CurrentScene::SelectPlaylist;
                                    app.input_mode = Some(InputMode::SelectPlaylist {
                                        selected_playlist: ListState::default()
                                            .with_selected(Some(0)),
                                    });
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            '+' => match app.curr_scene {
                                CurrentScene::Main => {
                                    app.curr_scene = CurrentScene::AddToPlaylist;
                                    if let Some(song) =
                                        app.player.as_ref().and_then(|p| p.get_selected_song())
                                    {
                                        app.input_mode = Some(InputMode::AddToPlaylist {
                                            selected_playlist: ListState::default()
                                                .with_selected(Some(0)),
                                            song_to_be_added: song,
                                        });
                                    } else {
                                        app.curr_scene = CurrentScene::Main;
                                        app.error_message =
                                            Some("Please first select a song to add".to_string());
                                    }
                                }
                                CurrentScene::AddToPlaylist | CurrentScene::SelectPlaylist => {
                                    app.input_mode = Some(InputMode::CreatePlaylist {
                                        playlist_name: String::default(),
                                    });
                                }
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            '-' => {
                                if let Some(input) = &mut app.input_mode {
                                    match input {
                                        InputMode::SelectPlaylist { selected_playlist } => {
                                            if let Some(selected_idx) = selected_playlist.selected()
                                            {
                                                app.curr_scene = CurrentScene::ConfirmRemove {
                                                    event: SceneEvent::RemovePlaylist {
                                                        playlist_index: selected_idx,
                                                    },
                                                };
                                            }
                                        }
                                        | InputMode::CreatePlaylist { .. }
                                        | InputMode::AddToPlaylist { .. } => {}
                                    }
                                } else if let Some(playlist) = app.get_default_playlist()
                                    && let Some(player) = &mut app.player
                                    && playlist.borrow().name != player.playlist.borrow().name
                                {
                                    let selected_idx = player.playlist.borrow().state.selected();
                                    if let Some(selected_idx) = selected_idx {
                                        app.curr_scene = CurrentScene::ConfirmRemove {
                                            event: SceneEvent::RemoveSong {
                                                song_index: selected_idx,
                                            },
                                        };
                                    } else {
                                        app.error_message =
                                            Some("First select song to remove".to_string());
                                    }
                                }
                            }
                            's' => match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        player.switch_shuffle();
                                    }
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            'r' => match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        player.switch_repeat();
                                    }
                                }
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            },
                            'y' => match &app.curr_scene {
                                CurrentScene::Exiting => {
                                    return Ok(true);
                                }
                                CurrentScene::ConfirmRemove { event } => match event {
                                    SceneEvent::RemoveSong { song_index } => {
                                        if let Some(player) = &mut app.player {
                                            player.playlist.borrow_mut().songs.remove(*song_index);
                                            app.curr_scene = CurrentScene::Main;
                                        }
                                    }
                                    SceneEvent::RemovePlaylist { playlist_index } => {
                                        match app.remove_playlist(*playlist_index) {
                                            Ok(_) => {}
                                            Err(_) => {
                                                app.error_message = Some(
                                                    "You cannot remove All Songs playlist"
                                                        .to_string(),
                                                )
                                            }
                                        }
                                        app.curr_scene = CurrentScene::SelectPlaylist;
                                    }
                                },
                                CurrentScene::Main
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist => {}
                            },
                            'n' => match &app.curr_scene {
                                CurrentScene::Exiting => {
                                    app.curr_scene = CurrentScene::Main
                                }
                                CurrentScene::ConfirmRemove { event } => match event {
                                    SceneEvent::RemoveSong { song_index: _ } => {
                                        app.curr_scene = CurrentScene::Main;
                                    }
                                    SceneEvent::RemovePlaylist { playlist_index: _ } => {
                                        app.curr_scene = CurrentScene::SelectPlaylist;
                                    }
                                },
                                CurrentScene::Main
                                | CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist => {}
                            },
                            'i' => {
                                app.switch_show_info();
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Esc => match app.curr_scene {
                        CurrentScene::AddToPlaylist => {
                            if let Some(input) = &app.input_mode {
                                match input {
                                    InputMode::SelectPlaylist { .. } 
                                    | InputMode::AddToPlaylist { .. } => {
                                        app.curr_scene = CurrentScene::Main;
                                        app.input_mode = None;
                                    }
                                    InputMode::CreatePlaylist { playlist_name: _ } => {
                                        if let Some(song) =
                                            app.player.as_ref().and_then(|p| p.get_selected_song())
                                        {
                                            app.input_mode = Some(InputMode::AddToPlaylist {
                                                song_to_be_added: song,
                                                selected_playlist: ListState::default()
                                                    .with_selected(Some(0)),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        CurrentScene::SelectPlaylist => {
                            if let Some(input) = &app.input_mode {
                                match input {
                                    InputMode::AddToPlaylist { .. } => {}
                                    InputMode::SelectPlaylist { .. } => {
                                        app.curr_scene = CurrentScene::Main;
                                        app.input_mode = None;
                                    }
                                    InputMode::CreatePlaylist { .. } => {
                                        app.curr_scene = CurrentScene::SelectPlaylist;
                                        app.input_mode = Some(InputMode::SelectPlaylist {
                                            selected_playlist: ListState::default()
                                                .with_selected(Some(0)),
                                        })
                                    }
                                }
                            }
                        }
                        CurrentScene::Main
                        | CurrentScene::ConfirmRemove { .. }
                        | CurrentScene::Exiting => {}
                    },
                    KeyCode::Up => {
                        if let Some(input_mode) = &mut app.input_mode {
                            match input_mode {
                                InputMode::SelectPlaylist { selected_playlist }
                                | InputMode::AddToPlaylist {
                                    selected_playlist, ..
                                } => {
                                    selected_playlist.select_previous();
                                }
                                InputMode::CreatePlaylist { .. } => {}
                            }
                        } else {
                            match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        player.playlist.borrow_mut().state.select_previous();
                                    }
                                }
                                CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(input_mode) = &mut app.input_mode {
                            match input_mode {
                                InputMode::SelectPlaylist { selected_playlist }
                                | InputMode::AddToPlaylist {
                                    selected_playlist, ..
                                } => {
                                    selected_playlist.select_next();
                                }
                                InputMode::CreatePlaylist { .. } => {}
                            }
                        } else {
                            match app.curr_scene {
                                CurrentScene::Main => {
                                    if let Some(player) = &mut app.player {
                                        player.playlist.borrow_mut().state.select_next();
                                    }
                                }
                                CurrentScene::AddToPlaylist
                                | CurrentScene::SelectPlaylist
                                | CurrentScene::ConfirmRemove { .. }
                                | CurrentScene::Exiting => {}
                            }
                        }
                    }
                    KeyCode::Left => match app.curr_scene {
                        CurrentScene::Main => {
                            if let Some(player) = &mut app.player {
                                match player.prev() {
                                    Ok(_) => {}
                                    Err(_) => {
                                        app.error_message = Some("Failed to play song".to_string())
                                    }
                                }
                            }
                        }
                        CurrentScene::AddToPlaylist
                        | CurrentScene::SelectPlaylist
                        | CurrentScene::ConfirmRemove { .. }
                        | CurrentScene::Exiting => {}
                    },
                    KeyCode::Right => match app.curr_scene {
                        CurrentScene::Main => {
                            if let Some(player) = &mut app.player {
                                match player.next() {
                                    Ok(_) => {}
                                    Err(_) => {
                                        app.error_message = Some("Failed to play song".to_string())
                                    }
                                }
                            }
                        }
                        CurrentScene::AddToPlaylist
                        | CurrentScene::SelectPlaylist
                        | CurrentScene::ConfirmRemove { .. }
                        | CurrentScene::Exiting => {}
                    },
                    KeyCode::Enter => {
                        if let Some(input) = app.input_mode.take() {
                            match &input {
                                InputMode::AddToPlaylist {
                                    song_to_be_added,
                                    selected_playlist,
                                } => {
                                    
                                    if let Some(selected_idx) = selected_playlist.selected()
                                    {
                                        let correct_index = selected_idx + 1; // Needs to be increased by one because the first playlist is the "all songs"
                                        app.add_song_to_playlist(
                                            correct_index,
                                            song_to_be_added.clone(),
                                        );
                                    }
                                    app.curr_scene = CurrentScene::Main;
                                    app.input_mode = None;
                                }
                                InputMode::SelectPlaylist { selected_playlist } => {
                                    if let Some(mut player) = app.player.take()
                                        && let Some(selected_idx) = selected_playlist.selected()
                                    {
                                        if let Some(playlist) =
                                            app.get_playlists().get(selected_idx)
                                        {
                                            player.set_playlist(playlist.clone());
                                            app.player = Some(player);
                                        } else {
                                            app.error_message = Some(
                                                "You have selected non existing playlist index"
                                                    .to_string(),
                                            );
                                        }
                                    }

                                    app.curr_scene = CurrentScene::Main;
                                    app.input_mode = None;
                                }
                                InputMode::CreatePlaylist { playlist_name } => {
                                    
                                    if playlist_name.is_empty() {
                                        app.error_message =
                                            Some("Playlist Name cannot be empty".to_string());
                                    } else {
                                        let playlist = Playlist::new(playlist_name.clone());
                                        match app.add_new_playlist(playlist) {
                                            Ok(_) => {}
                                            Err(_) => {
                                                app.error_message = Some(format!(
                                                    "Playlist {} already existed",
                                                    playlist_name.as_str()
                                                ))
                                            }
                                        }
                                    }
                                    
                                    match app.curr_scene {
                                        CurrentScene::AddToPlaylist => {
                                            if let Some(song) = app
                                                .player
                                                .as_ref()
                                                .and_then(|p| p.get_selected_song())
                                            {
                                                app.input_mode = Some(InputMode::AddToPlaylist {
                                                    song_to_be_added: song,
                                                    selected_playlist: ListState::default()
                                                        .with_selected(Some(0)),
                                                });
                                            }
                                        }
                                        CurrentScene::SelectPlaylist => {
                                            app.input_mode = Some(InputMode::SelectPlaylist {
                                                selected_playlist: ListState::default()
                                                    .with_selected(Some(0)),
                                            }); // Go back to previous select list
                                        }
                                        CurrentScene::Main
                                        | CurrentScene::ConfirmRemove { .. }
                                        | CurrentScene::Exiting => {}
                                    }
                                }
                            }
                            app.input_mode = Some(input);
                        }
                    }
                    KeyCode::Tab => {
                        if let Some(input) = &mut app.input_mode {
                            match input {
                                InputMode::AddToPlaylist { .. }
                                | InputMode::SelectPlaylist { .. }
                                | InputMode::CreatePlaylist { .. } => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else if let Some(player) = &mut app.player
            && player.finished_playing()
            && player.is_playing()
        {
            match player.next() {
                Ok(_) => {}
                Err(_) => app.error_message = Some("Failed to play song".to_string()),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    Logger::try_with_str("debug")
        .unwrap()
        .log_to_file(FileSpec::default().directory("dev_logs"))
        .start()
        .unwrap();

    // create app and run it
    let mut app = App::new();
    app.init()?;
    let _ = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
