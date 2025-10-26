// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use gtk::{gdk::DragAction, glib::GString, prelude::*, DestDefaults, TargetEntry, TargetFlags};
use serde::Serialize;
use std::{cell::Cell, path::PathBuf, rc::Rc};
use tauri::{App, Emitter, Manager, PhysicalPosition};

#[derive(Serialize)]
struct DragDropPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<Vec<PathBuf>>,
    position: PhysicalPosition<f64>,
}

pub fn connect(app: &mut App) {
    let window = app.get_webview_window("main").unwrap();
    let content = window.default_vbox().unwrap();

    for child in content.children() {
        child.drag_dest_unset();
    }

    content.drag_dest_set(
        DestDefaults::ALL,
        &[TargetEntry::new(
            "application/vnd.portal.filetransfer",
            TargetFlags::empty(),
            0,
        )],
        DragAction::COPY,
    );

    let entered = Rc::new(Cell::new(false));

    {
        let window = window.clone();
        let entered = entered.clone();
        content.connect_drag_motion(move |_, _, x, y, _| {
            if !entered.get() {
                entered.set(true);

                let payload = DragDropPayload {
                    paths: Some(vec![]),
                    position: PhysicalPosition::new(x as _, y as _),
                };
                window.emit("tauri://drag-enter", &payload).unwrap();
            } else {
                let payload = DragDropPayload {
                    paths: None,
                    position: PhysicalPosition::new(x as _, y as _),
                };
                window.emit("tauri://drag-over", &payload).unwrap();
            }
            true
        });
    }

    {
        let window = window.clone();
        content.connect_drag_data_received(move |_, ctx, x, y, data, _, time| {
            let paths = data
                .uris()
                .iter()
                .map(path_buf_from_uri)
                .collect::<Vec<_>>();

            let payload = DragDropPayload {
                paths: Some(paths),
                position: PhysicalPosition::new(x as _, y as _),
            };
            window.emit("tauri://drag-drop", &payload).unwrap();

            ctx.drag_finish(true, false, time);
        });
    }

    {
        let window = window.clone();
        let entered = entered.clone();
        content.connect_drag_leave(move |_, _, _| {
            entered.set(false);

            window.emit("tauri://drag-leave", &{}).unwrap();
        });
    }
}

fn path_buf_from_uri(gstr: &GString) -> PathBuf {
    let path = gstr.as_str();
    let path = path.strip_prefix("file://").unwrap_or(path);
    let path = percent_encoding::percent_decode(path.as_bytes())
        .decode_utf8_lossy()
        .to_string();
    PathBuf::from(path)
}
