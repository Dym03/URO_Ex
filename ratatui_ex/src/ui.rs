use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Duration};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::{
    app::{App, CurrentScene, InputMode, SceneEvent},
    audio::{Playlist, Song},
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let mut constraints = vec![
        Constraint::Max(3),  // title
        Constraint::Fill(1), // main content
        Constraint::Max(3),  // footer
    ];

    if app.error_message.is_some() {
        constraints.insert(2, Constraint::Percentage(20));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());

    render_title(frame, chunks[0]);
    render_content(frame, chunks[1], app);

    if app.error_message.is_some() {
        render_error(frame, chunks[2], app);
    }

    render_footer(frame, *chunks.last().unwrap(), app);

    if let CurrentScene::Exiting = app.curr_scene {
        render_confirm_popup(
            frame,
            "Do you really want to leave ?",
            "( Y to leave | N to stay )",
        );
    }
}

fn render_error(frame: &mut Frame, chunks: Rect, app: &mut App) {
    if let Some(error_message) = &app.error_message {
        let error_block = Block::default()
            .style(Color::Red)
            .title("Error Block")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let error_string = error_message.clone() + "\n Press any button to close the error message";

        let error_text = Paragraph::new(error_string)
            .block(error_block)
            .alignment(Alignment::Center);
        frame.render_widget(error_text, chunks);
    }
}

fn render_title(frame: &mut Frame, area: Rect) {
    let block = Block::default().borders(Borders::ALL);
    let title = Paragraph::new("Rust song Player")
        .style(Style::default().fg(Color::Green))
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &mut App) {
    match &app.curr_scene {
        CurrentScene::Main => {
            let mut constraints = vec![];
            constraints.push(Constraint::Fill(1));

            if let Some(player) = app.player.as_mut() {
                let song_progress = player.has_playing_song();
                if song_progress {
                    constraints.push(Constraint::Max(3));
                }

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(area);

                let song_elapsed_time = player.get_song_elapsed_duration();
                let mut playlist_guard = player.playlist.borrow_mut();

                let playlist = &mut *playlist_guard;
                let songs = &playlist.songs;
                let state = &mut playlist.state;

                let mut constraints_for_songs = VecDeque::new();
                constraints_for_songs.push_back(Constraint::Fill(3));
                if player.has_songs_in_queue() {
                    constraints_for_songs.push_back(Constraint::Fill(1));
                }

                if app.show_info {
                    constraints_for_songs.push_front(Constraint::Fill(1));
                }

                let chunks_for_songs = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints_for_songs)
                    .split(chunks[0]);

                if app.show_info {
                    render_info(frame, chunks_for_songs[0]);
                    render_song_list(frame, chunks_for_songs[1], songs, state);
                } else {
                    render_song_list(frame, chunks_for_songs[0], songs, state);
                }

                if player.has_songs_in_queue() {
                    let songs_in_queue: Vec<String> = player
                        .get_song_queue()
                        .iter()
                        .map(|song| song.name.chars().take(40).collect())
                        .collect();
                    render_song_queue(frame, *chunks_for_songs.last().unwrap(), songs_in_queue);
                }

                if song_progress && let Some(playing_song) = player.get_current_playing_song() {
                    render_song_progress(frame, chunks[1], playing_song, song_elapsed_time);
                }
            }
        }
        CurrentScene::AddToPlaylist => {
            render_add_to_playlist_ui(frame, app);
        }
        CurrentScene::SelectPlaylist => {
            render_select_playlist(frame, app);
        }
        CurrentScene::ConfirmRemove { event } => {
            let (option, hint) = match event {
                SceneEvent::RemoveSong { song_index } => {
                    
                    if let Some(player) = &app.player && let Some(song) = player.get_songs().get(*song_index){
                        (format!("Do you really want to remove {} song ?", song.name),
                        "Y to remove | N to go back")
                    } else {
                        ("".to_string(), "")
                    }
                    
                }
                SceneEvent::RemovePlaylist { playlist_index } => {
                    if let Some(playlist) = app.get_playlists().get(*playlist_index) {
                        let playlist_name = playlist.borrow().name.clone();
                        (
                        format!("Do you really want to remove {playlist_name} playlist ?"),
                        "Y to remove | N to go back",
                        )
                    } else {
                        ("".to_string(), "")
                    }
                }
            };

            render_confirm_popup(frame, &option, hint);
        }
        CurrentScene::Exiting => {}
    }
}

fn render_info(frame: &mut Frame, area: Rect) {
    let mut list_items = Vec::<ListItem>::new();

    let commands = [
        "add to playlist | +",
        "remove from playlist | -",
        "change playlist | p",
        "add to queue | a",
        "download | d",
        "quit | q",
    ];

    for command in commands.iter() {
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                command.to_string(),
                Style::default().fg(Color::Yellow),
            ))
            .alignment(Alignment::Center),
        ));
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Commands")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().fg(Color::Black));

    frame.render_widget(list, area);
}

fn render_select_playlist(frame: &mut Frame, app: &mut App) {
    if let Some(mut editing) = app.input_mode.take() {
        let area = centered_rect(60, 60, frame.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Fill(1),
                Constraint::Percentage(20),
            ])
            .split(area);

        let block = Block::default().borders(Borders::ALL);
        let title = Paragraph::new("Select playlist")
            .style(Style::default().fg(Color::Green))
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(title, chunks[0]);

        match &mut editing {
            InputMode::AddToPlaylist {
                selected_playlist: _,
                song_to_be_added: _,
            } => {}
            InputMode::SelectPlaylist { selected_playlist } => {
                
                let playlists = app.get_playlists();
                render_playlist_list(frame, chunks[1], playlists, selected_playlist);
                

                let return_text_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL);

                let return_text: Paragraph<'_> = Paragraph::new(
                    "Press Esc to return | Enter to select\nRemove Playlist | -\nAdd Playlist | +",
                )
                .alignment(Alignment::Center)
                .block(return_text_block);

                frame.render_widget(return_text, chunks[2]);
            }
            InputMode::CreatePlaylist { playlist_name } => {
                let mut playlist_name_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Enter Playlist Name")
                    .borders(Borders::ALL);

                let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

                playlist_name_block = playlist_name_block.style(active_style);
                let playlist_name_text = Paragraph::new(playlist_name.clone())
                    .alignment(Alignment::Center)
                    .block(playlist_name_block);
                frame.render_widget(playlist_name_text, chunks[1]);

                let return_text_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL);

                let return_text: Paragraph<'_> =
                    Paragraph::new("Press Esc to go back | Enter to create")
                        .alignment(Alignment::Center)
                        .block(return_text_block);

                frame.render_widget(return_text, chunks[2]);
            }
        };

        app.input_mode = Some(editing); // Put the editing back to the app state so it can be used in event handling
    }
}

fn render_add_to_playlist_ui(frame: &mut Frame, app: &mut App) {
    if let Some(mut editing) = app.input_mode.take() {
        let area = centered_rect(60, 60, frame.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Fill(1),
                Constraint::Percentage(20),
            ])
            .split(area);

        let block = Block::default().borders(Borders::ALL);
        let title = Paragraph::new("Select playlist")
            .style(Style::default().fg(Color::Green))
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(title, chunks[0]);

        match &mut editing {

            InputMode::AddToPlaylist {
                selected_playlist,
                song_to_be_added: _,
            } => {
                let playlists = app.get_playlists();
                let playlists_to_render = if !playlists.is_empty() {
                    &playlists[1..]
                } else {
                    &[]
                };
                render_playlist_list(frame, chunks[1], playlists_to_render, selected_playlist);
            

                let return_text_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL);

                let return_text: Paragraph<'_> =
                    Paragraph::new("Press Esc to return | Enter to select\n Create Playlist | +")
                        .alignment(Alignment::Center)
                        .block(return_text_block);

                frame.render_widget(return_text, chunks[2]);
            }
            InputMode::SelectPlaylist {
                selected_playlist: _,
            } => {}
            InputMode::CreatePlaylist { playlist_name } => {
                let mut playlist_name_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .title("Enter Playlist Name")
                    .borders(Borders::ALL);

                let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

                playlist_name_block = playlist_name_block.style(active_style);
                let playlist_name_text = Paragraph::new(playlist_name.clone())
                    .alignment(Alignment::Center)
                    .block(playlist_name_block);
                frame.render_widget(playlist_name_text, chunks[1]);

                let return_text_block = Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL);

                let return_text: Paragraph<'_> =
                    Paragraph::new("Press Esc to go back | Enter to create playlist")
                        .alignment(Alignment::Center)
                        .block(return_text_block);

                frame.render_widget(return_text, chunks[2]);
            } // !TODO
        };
        app.input_mode = Some(editing); 
    }
}

fn render_song_queue(frame: &mut Frame<'_>, area: Rect, songs_in_queue: Vec<String>) {
    let mut list_items = Vec::<ListItem>::new();

    for song in songs_in_queue.iter() {
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                format!("{: <25}", song),
                Style::default().fg(Color::Yellow),
            ))
            .alignment(Alignment::Center),
        ));
    }

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Queue")
            .title_alignment(Alignment::Center),
    );

    frame.render_widget(list, area);
}

fn render_song_progress(
    frame: &mut Frame<'_>,
    area: Rect,
    playing_song: Rc<Song>,
    song_elapsed_time: Duration,
) {
    if let Some(total_song_duration) = playing_song.get_duration() {
        let ratio = song_elapsed_time.as_secs_f64() / total_song_duration.as_secs_f64();
        let ratio = ratio.min(1.0); // clamp to 1.0 when done

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!("Now Playing: {}", playing_song.name))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .label(format!(
                "{:02}:{:02} / {:02}:{:02}",
                song_elapsed_time.as_secs() / 60,
                song_elapsed_time.as_secs() % 60,
                total_song_duration.as_secs() / 60,
                total_song_duration.as_secs() % 60
            ))
            .ratio(ratio);

        frame.render_widget(gauge, area);
    }
}

fn render_song_list(frame: &mut Frame, area: Rect, songs: &[Rc<Song>], state: &mut ListState) {
    let mut list_items = Vec::<ListItem>::new();

    for song in songs.iter() {
        let duration = match song.get_duration() {
            Some(duration) => {
                format!("{}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
            }
            None => "Unknown".to_string(),
        };
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                format!("{: <25} : {}", song.name, duration),
                Style::default().fg(Color::Yellow),
            ))
            .alignment(Alignment::Center),
        ));
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Songs")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightGreen)) // selected style
        .highlight_symbol(">>"); // optional symbol for the selected item;

    frame.render_stateful_widget(list, area, state);
}

fn render_playlist_list(
    frame: &mut Frame,
    area: Rect,
    playlists: &[Rc<RefCell<Playlist>>],
    state: &mut ListState,
) {
    let mut list_items = Vec::<ListItem>::new();

    for playlist in playlists.iter() {
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                format!("{: <15}", playlist.borrow().name),
                Style::default().fg(Color::Yellow),
            ))
            .alignment(Alignment::Center),
        ));
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Playlists")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightGreen)) // selected style
        .highlight_symbol(">>"); // optional symbol for the selected item;

    frame.render_stateful_widget(list, area, state);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let navigation_options = match app.curr_scene {
        CurrentScene::Main => {
            if let Some(player) = &app.player {
                let shuffle = if player.get_shuffle() {
                    "shuffle off | s"
                } else {
                    "shuffle on | s"
                };
                let repeat = if player.get_repeat() {
                    "repeat off | r"
                } else {
                    "repeat on | r"
                };

                let play = if player.is_playing() && !player.has_different_selected() {
                    "pause | space"
                } else {
                    "play | space"
                };

                let info = if app.show_info {
                    "close commands | i"
                } else {
                    "show commands | i"
                };

                vec![shuffle, "prev | <-", play, "next | ->", repeat, info]
            } else {
                vec!["prev | <-", "play | space ", "next | ->", "quit | q"]
            }
        }
        CurrentScene::Exiting
        | CurrentScene::AddToPlaylist
        | CurrentScene::SelectPlaylist
        | CurrentScene::ConfirmRemove { .. } => vec![],
    };

    let constraints: Vec<Constraint> = navigation_options
        .iter()
        .map(|_| Constraint::Fill(1))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, nav_opt) in navigation_options.iter().enumerate() {
        let widget = Paragraph::new(*nav_opt)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(widget, chunks[i]);
    }
}

fn render_confirm_popup(frame: &mut Frame, block_text: &str, options_text: &str) {
    let area = centered_rect(60, 25, frame.area());

    let block = Block::default()
        .title(block_text)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let text = Text::styled(options_text, Style::default().fg(Color::Red));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, frame.area());
    frame.render_widget(paragraph, area);
}
