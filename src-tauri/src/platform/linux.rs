use std::fs::File;

use ashpd::desktop::open_uri::{OpenDirectoryRequest, OpenFileRequest};

pub async fn open_file_impl(file_path: String) {
    let file = File::open(&file_path).unwrap();
    OpenFileRequest::default()
        .ask(true)
        .send_file(&file)
        .await
        .unwrap();
}

pub async fn reveal_file_impl(file_path: String) {
    let file = File::open(&file_path).unwrap();
    OpenDirectoryRequest::default().send(&file).await.unwrap();
}
