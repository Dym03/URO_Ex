use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, io};

use crate::audio::{Player, Playlist, Song};
use ratatui::widgets::ListState;

const SONGS_DIR: &str = "songs";

pub enum SceneEvent {
    RemoveSong { song_index: usize },
    RemovePlaylist { playlist_index: usize },
}

pub enum CurrentScene {
    Main,
    AddToPlaylist,
    SelectPlaylist,
    ConfirmRemove { event: SceneEvent },
    Exiting,
}

pub enum InputMode {
    AddToPlaylist {
        song_to_be_added: Rc<Song>,
        selected_playlist: ListState,
    },
    SelectPlaylist {
        selected_playlist: ListState,
    },
    CreatePlaylist {
        playlist_name: String,
    },
}

pub enum DownloadError {
    VideoNotFound { url: String },
    DownloadFailed { url: String },
}

pub enum AppError {
    PlaylistExists,
    PlaylistLoadFailed,
    BadRemoveRequest,
}

pub struct App {
    pub curr_scene: CurrentScene,
    pub playlists: Vec<Rc<RefCell<Playlist>>>,
    pub input_mode: Option<InputMode>,
    pub player: Option<Player>,
    // pub downloader: Option<Downloader>,
    pub error_message: Option<String>,
    pub show_info: bool,
    pub animation_tick: usize,
}

impl App {
    pub fn new() -> App {
        App {
            curr_scene: CurrentScene::Main,
            input_mode: None,
            player: None,
            error_message: None,
            // downloader: None,
            show_info: false,
            playlists: Vec::new(),
            animation_tick: 0,
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.create_file_structure()?;

        match Playlist::load_playlist(SONGS_DIR.into()) {
            Ok(playlist) => {
                // First playlist holds all songs from the directory other playlist will just hold references to songs from this playlist
                log::debug!("{playlist:?}");
                self.playlists.push(Rc::new(RefCell::new(playlist)));
                self.player = Some(Player::new(self.playlists[0].clone()));
                Ok(())
            }
            Err(_) => {
                println!("Failed to load playlist, creating new one");
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to load playlist, creating new one",
                ))
            }
        }
    }

    pub fn create_file_structure(&self) -> io::Result<()> {
        let dirs = vec![SONGS_DIR];
        for dir in dirs {
            if let Ok(exists) = fs::exists(dir)
                && !exists
            {
                fs::create_dir(dir)?
            }
        }
        Ok(())
    }

    pub fn switch_show_info(&mut self) {
        self.show_info = !self.show_info;
    }

    pub fn get_default_playlist(&self) -> Option<Rc<RefCell<Playlist>>> {
        if !self.playlists.is_empty() {
            return Some(self.playlists[0].clone());
        }
        None
    }

    pub fn get_playlists(&self) -> &Vec<Rc<RefCell<Playlist>>> {
        &self.playlists
    }

    pub fn add_song_to_playlist(&mut self, playlist_idx: usize, song: Rc<Song>) {
        if let Some(playlist) = self.playlists.get(playlist_idx) {
            playlist.borrow_mut().add_song(song);
        }
    }

    pub fn add_new_playlist(&mut self, playlist: Playlist) -> Result<(), AppError> {
        if self
            .playlists
            .iter()
            .any(|p| p.borrow().name == playlist.name)
        {
            return Err(AppError::PlaylistExists);
        }
        self.playlists.push(Rc::new(RefCell::new(playlist)));
        Ok(())
    }

    pub fn remove_playlist(&mut self, playlist_idx: usize) -> Result<(), AppError> {
        if playlist_idx == 0 {
            return Err(AppError::BadRemoveRequest); // We cannot remove the all songs playlist
        } else if playlist_idx < self.playlists.len() {
            self.playlists.remove(playlist_idx);
        } else {
            return Err(AppError::BadRemoveRequest); // This shouldn't happen as the ListState is connected to the list of playlist but in the case it happens
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
