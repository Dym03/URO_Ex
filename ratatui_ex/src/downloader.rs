// use std::{path::PathBuf, sync::mpsc::SendError, thread};

// use tokio::runtime::Runtime;
// use yt_dlp::{
//     Downloader as YtDownloader,
//     client::Libraries,
//     model::{AudioCodecPreference, AudioQuality},
// };

// #[derive(Debug)]
// pub enum DownloadEvent {
//     Succesful(String),
//     Failed(String),
// }

// pub struct Downloader {
//     tx: std::sync::mpsc::SyncSender<String>,
//     pub rx: std::sync::mpsc::Receiver<DownloadEvent>,
//     handle: thread::JoinHandle<()>,
// }

// impl Downloader {
//     pub fn new(root_dir: PathBuf) -> Downloader {
//         let (url_tx, url_rx) = std::sync::mpsc::sync_channel::<String>(10);
//         let (event_tx, event_rx) = std::sync::mpsc::sync_channel(10);

//         let handle = thread::spawn(move || {
//             let rt = Runtime::new().expect("Failed to create Tokio runtime");

//             rt.block_on(async {
//                 let libraries_dir = PathBuf::from("libs");

//                 let youtube = libraries_dir.join("yt-dlp");
//                 let ffmpeg: PathBuf = libraries_dir.join("ffmpeg");

//                 let fetcher = if youtube.exists() && ffmpeg.exists() {
//                     log::debug!(
//                         "Found existing binaries in {:?}. using them.",
//                         libraries_dir
//                     );

//                     let libraries = Libraries::new(youtube, ffmpeg);
//                     YtDownloader::for_youtube(libraries, &root_dir).await
//                 } else {
//                     log::debug!("Binaries missing. Downloading to {:?}...", libraries_dir);

//                     // Use the helper that downloads files (requires internet)
//                     YtDownloader::with_new_binaries(libraries_dir, &root_dir).await
//                 };

//                 let fetcher = match fetcher {
//                     Ok(r) => r,
//                     Err(e) => {
//                         let _ =
//                             event_tx.send(DownloadEvent::Failed(format!("Init failed: {:?}", e)));
//                         log::debug!("Init failed");
//                         return; // Exit thread if init fails
//                     }
//                 };

//                 while let Ok(url) = url_rx.recv() {
//                     log::debug!("received {url}");
//                     if let Some(clean_url) = clean_url(url) {
//                         let info = fetcher.fetch_video_infos(clean_url.clone()).await;
//                         let file_name = match info {
//                             Ok(info) => sanitize_filename(&info.title),
//                             Err(_) => clean_url.clone(), // Fallback
//                         };

//                         let output_path = format!("{}.m4a", file_name);

//                         match fetcher
//                             .download_audio_stream_with_quality(
//                                 &clean_url,
//                                 &output_path,
//                                 AudioQuality::High, // low
//                                 AudioCodecPreference::AAC,
//                             )
//                             .await
//                         {
//                             Ok(_) => {
//                                 let output_path = root_dir.join(output_path);
//                                 let _ = event_tx.send(DownloadEvent::Succesful(String::from(
//                                     output_path.to_string_lossy(),
//                                 )));
//                                 log::debug!("Downloaded succesfully");
//                             }
//                             Err(err) => {
//                                 let _ = event_tx.send(DownloadEvent::Failed(format!(
//                                     "Failed to download file with url {clean_url}"
//                                 )));
//                                 log::debug!("Download failed");
//                                 log::debug!("{}", err);
//                             }
//                         };
//                     }
//                 }

//                 log::debug!("Ended correctly");
//             })
//         });

//         Downloader {
//             tx: url_tx,
//             rx: event_rx,
//             handle,
//         }
//     }

//     pub fn queue_url(&self, url: String) -> Result<(), SendError<String>> {
//         self.tx.send(url)
//     }

//     pub fn close(self) {
//         drop(self.tx);
//         let _ = self.handle.join();
//     }
// }

// fn clean_url(url: String) -> Option<String> {
//     if let Some((url, _rest)) = url.split_once("&") {
//         Some(url.to_string())
//     } else {
//         Some(url)
//     }
// }

// fn sanitize_filename(name: &str) -> String {
//     name.replace(
//         |c: char| !c.is_alphanumeric() && c != ' ' && c != '-' && c != '_',
//         "",
//     )
// }
