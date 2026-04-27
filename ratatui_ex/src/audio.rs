use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;
use std::{fs, vec};

use ratatui::widgets::ListState;
use rodio::OutputStreamBuilder;
use rodio::Sink;
use rodio::{Decoder, OutputStream, Source};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Song {
    path: PathBuf,
    pub name: String,
    length: Option<Duration>,
}

impl Song {
    pub fn new_from_path(file_path: PathBuf) -> Option<Song> {
        log::debug!(
            "Is path: {}, Is Valid song path {}",
            file_path.is_file(),
            is_valid_song_path(&file_path)
        );
        if file_path.is_file() && is_valid_song_path(&file_path) {
            let file = BufReader::new(File::open(file_path.clone()).unwrap());
            let source = Decoder::try_from(file);
            let song_duration = match source {
                Ok(source) => source.total_duration(),
                Err(_) => return None,
            };

            Some(Song {
                path: file_path.clone(),
                name: file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                length: song_duration,
            })
        } else {
            None
        }
    }

    fn load_source(&self) -> Option<Decoder<BufReader<File>>> {
        let file = BufReader::new(File::open(self.path.clone()).unwrap());
        log::debug!("Playing {:?} {}", self.path.clone(), file.capacity());
        let source = Decoder::try_from(file);
        if let Ok(source) = source {
            Some(source)
        } else {
            log::error!("Failed to decode file {:?}", self.path.clone());
            None
        }
    }

    pub fn get_duration(&self) -> Option<Duration> {
        self.length
    }
}

pub fn is_valid_song_path(path: &Path) -> bool {
    let valid_song_suffixes = ["mp3", "wav", "m4a"];
    if let Some(suffix) = path.extension() {
        if let Some(suffix_str) = suffix.to_str() {
            valid_song_suffixes.contains(&suffix_str.to_lowercase().as_str())
        } else {
            false
        }
    } else {
        false
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Rc<Song>>,
    pub state: ListState,
}

pub enum PlaylistError {
    LoadFailed,
    IoError(io::Error),
}

impl From<io::Error> for PlaylistError {
    fn from(err: io::Error) -> PlaylistError {
        PlaylistError::IoError(err)
    }
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist {
            name,
            songs: vec![],
            state: ListState::default(),
        }
    }

    pub fn load_playlist(path: PathBuf) -> Result<Playlist, PlaylistError> {
        let state: ListState = ListState::default();
        let mut result = Playlist {
            name: "All Songs".to_string(),
            songs: vec![],
            state,
        };
        log::debug!("Reading dir: {:#?}", path);
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let dir_entry = entry?;
                let file_path = dir_entry.path();
                log::debug!("File: {:#?}", file_path);

                match Song::new_from_path(file_path.clone()) {
                    Some(song) => result.songs.push(Rc::new(song)),
                    None => {
                        log::debug!("Failed to load song {:?}", file_path);
                    }
                }
            }
        }
        if !result.songs.is_empty() {
            result.state.select(Some(0));
        }
        Ok(result)
    }

    pub fn add_song(&mut self, song: Rc<Song>) {
        self.songs.push(song);
    }
}

pub enum PlayerError {
    FailedToLoadSource,
    AddToQueueFailed,
}

pub struct Player {
    pub playlist: Rc<RefCell<Playlist>>,
    playing_song: Option<Rc<Song>>,
    song_queue: Sink,
    to_be_played: VecDeque<Rc<Song>>,
    stream_handle: OutputStream, // We have to keep the stream alive as long as we want to play the music
    shuffle: bool,
    repeat: bool,
}

impl Player {
    pub fn new(playlist: Rc<RefCell<Playlist>>) -> Player {
        let stream_handle =
            OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        Player {
            playlist,
            playing_song: None,
            song_queue: sink,
            stream_handle,
            shuffle: false,
            repeat: false,
            to_be_played: vec![].into(),
        }
    }

    pub fn finished_playing(&self) -> bool {
        self.song_queue.empty()
    }

    pub fn get_repeat(&self) -> bool {
        self.repeat
    }

    pub fn switch_repeat(&mut self) {
        self.repeat = !self.repeat;
    }

    pub fn get_shuffle(&self) -> bool {
        self.shuffle
    }

    pub fn switch_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
    }

    pub fn get_songs(&self) -> Ref<Vec<Rc<Song>>> {
        Ref::map(self.playlist.borrow(), |p| &p.songs)
    }

    pub fn get_mut_songs(&mut self) -> RefMut<Vec<Rc<Song>>> {
        RefMut::map(self.playlist.borrow_mut(), |p| &mut p.songs)
    }

    pub fn get_playlist_state(&mut self) -> RefMut<ListState> {
        RefMut::map(self.playlist.borrow_mut(), |p| &mut p.state)
    }

    pub fn get_current_playing_song(&self) -> Option<Rc<Song>> {
        self.playing_song.clone()
    }

    pub fn get_selected_song(&self) -> Option<Rc<Song>> {
        let playlist = self.playlist.borrow();
        playlist
            .state
            .selected()
            .and_then(|index| playlist.songs.get(index))
            .cloned()
    }

    pub fn has_playing_song(&self) -> bool {
        self.playing_song.is_some()
    }

    pub fn is_playing(&self) -> bool {
        self.has_playing_song() && !self.song_queue.is_paused()
    }

    pub fn pause(&mut self) {
        self.song_queue.pause();
    }

    pub fn has_songs_in_queue(&self) -> bool {
        !self.to_be_played.is_empty()
    }

    pub fn get_song_queue(&self) -> &VecDeque<Rc<Song>> {
        &self.to_be_played
    }

    //Is playing song the same as the selected one
    pub fn has_different_selected(&self) -> bool {
        if let Some(selected_idx) = self.playlist.borrow().state.selected() {
            if self.has_playing_song() {
                self.playing_song != self.playlist.borrow().songs.get(selected_idx).cloned()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_song_elapsed_duration(&self) -> Duration {
        self.song_queue.get_pos()
    }

    pub fn set_playlist(&mut self, playlist: Rc<RefCell<Playlist>>) {
        // self.clear_queue();
        self.playlist = playlist;
    }

    pub fn next(&mut self) -> Result<(), PlayerError> {
        if self.repeat && self.to_be_played.is_empty() {
            if let Some(song) = self.get_selected_song() {
                self.play_song(song)?;
            }
            return Ok(());
        }
        if self.shuffle && self.to_be_played.is_empty() {
            let random_idx: usize = rand::random_range(0..self.get_songs().len());
            log::debug!("Selected {random_idx}");
            self.playlist.borrow_mut().state.select(Some(random_idx));
            self.play()?;
            return Ok(());
        }

        if !self.to_be_played.is_empty() {
            self.song_queue.skip_one();
            let song_to_be_played = self.to_be_played.pop_front().unwrap();
            self.playing_song = Some(song_to_be_played.clone());
            self.play_song(song_to_be_played)?;
            Ok(())
        } else {
            // self.playlist.borrow_mut().state.select(self.playing_idx); // Original solution with indeces, now as we are holding a song, this is problematic if we switch playlists
            self.playlist.borrow_mut().state.select_next();
            self.play()?;
            Ok(())
        }
    }

    pub fn prev(&mut self) -> Result<(), PlayerError> {
        self.playlist.borrow_mut().state.select_previous();
        self.play()?;
        Ok(())
    }

    pub fn add_to_queue(&mut self) -> Result<(), PlayerError> {
        if self.song_queue.empty()
            && let Some(song) = self.get_selected_song()
        {
            self.playing_song = Some(song.clone());
            self.play_song(song)?;
        } else if let Some(song) = self.get_selected_song() {
            self.to_be_played.push_back(song);
        } else {
            return Err(PlayerError::AddToQueueFailed);
        }

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.song_queue.clear();
        self.to_be_played.clear();
    }

    pub fn play_song(&mut self, song: Rc<Song>) -> Result<(), PlayerError> {
        let source = song.load_source();
        match source {
            Some(source) => {
                self.song_queue.append(source);
                Ok(())
            }
            None => Err(PlayerError::FailedToLoadSource),
        }
    }

    pub fn play(&mut self) -> Result<(), PlayerError> {
        self.song_queue.play();

        let selected_song = self.get_selected_song();

        match (&self.playing_song, selected_song) {
            (Some(playing_song), Some(selected_song)) => {
                if *playing_song != selected_song {
                    // Playing different song than before
                    self.song_queue.stop();
                    self.playing_song = None;

                    let source = selected_song.load_source();
                    match source {
                        Some(source) => {
                            self.song_queue.append(source);
                            self.playing_song = Some(selected_song);
                            return Ok(());
                        }
                        None => {
                            return Err(PlayerError::FailedToLoadSource);
                        }
                    }
                } else {
                    let source = selected_song.load_source();
                    match source {
                        Some(source) => {
                            self.song_queue.append(source);
                            return Ok(());
                        }
                        None => {
                            return Err(PlayerError::FailedToLoadSource);
                        }
                    }
                }
            }
            (None, Some(selected_song)) => {
                let source = selected_song.load_source();
                match source {
                    Some(source) => {
                        self.song_queue.append(source);
                        self.playing_song = Some(selected_song);
                        return Ok(());
                    }
                    None => {
                        return Err(PlayerError::FailedToLoadSource);
                    }
                }
            }
            (Some(_), None) => {}
            (None, None) => {}
        }
        Ok(())
    }
}
