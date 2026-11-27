use std::path::PathBuf;

use dioxus::prelude::*;
use docuforge_core::{JobEvent, JobQueue, start_pdf_merge};
use rfd::AsyncFileDialog;

#[component]
pub fn MergePanel() -> Element {
    let mut inputs = use_signal(Vec::<PathBuf>::new);
    let mut dialog_open = use_signal(|| false);
    let mut processing = use_signal(|| false);
    let mut status = use_signal(String::new);
    let queue = use_hook(JobQueue::default);

    let unavailable = dialog_open() || processing();
    let paths = inputs.read();
    let has_inputs = !paths.is_empty();
    let can_merge = paths.len() >= 2 && !unavailable;

    rsx! {
        section { class: "tool-panel", aria_labelledby: "merge-title",
            h2 { id: "merge-title", "Merge PDF" }
            p { "Add at least two PDFs and arrange them in the order you want." }
            div { class: "tool-actions",
                button {
                    r#type: "button",
                    disabled: unavailable,
                    onclick: move |_| async move {
                        if *dialog_open.peek() || *processing.peek() {
                            return;
                        }
                        dialog_open.set(true);
                        let files = AsyncFileDialog::new()
                            .set_title("Add source PDFs")
                            .add_filter("PDF", &["pdf"])
                            .pick_files()
                            .await;
                        if let Some(files) = files {
                            inputs.write().extend(files.into_iter().map(|file| file.path().to_owned()));
                            status.set(String::new());
                        }
                        dialog_open.set(false);
                    },
                    "Add PDFs"
                }
                button {
                    r#type: "button",
                    class: "primary-action",
                    disabled: !can_merge,
                    onclick: move |_| {
                        let queue = queue.clone();
                        async move {
                            if *dialog_open.peek() || *processing.peek() || inputs.peek().len() < 2 {
                                return;
                            }
                            dialog_open.set(true);
                            let destination = AsyncFileDialog::new()
                                .set_title("Save merged PDF")
                                .set_file_name("merged.pdf")
                                .add_filter("PDF", &["pdf"])
                                .save_file()
                                .await;
                            dialog_open.set(false);
                            if let Some(file) = destination {
                                let output = file.path().to_owned();
                                let sources = inputs.peek().clone();
                                processing.set(true);
                                status.set("Queued".into());
                                let result = run_merge(&queue, sources, output, status).await;
                                status.set(result);
                                processing.set(false);
                            }
                        }
                    },
                    "Merge and save"
                }
            }
            if has_inputs {
                ol { class: "merge-sources",
                    for (index, path) in paths.iter().enumerate() {
                        li {
                            span { class: "source-name", title: "{path.display()}",
                                "{path.file_name().unwrap_or(path.as_os_str()).to_string_lossy()}"
                            }
                            div { class: "source-actions",
                                button {
                                    r#type: "button",
                                    aria_label: "Move source {index + 1} up",
                                    disabled: unavailable || index == 0,
                                    onclick: move |_| {
                                        let mut sources = inputs.write();
                                        if index > 0 && index < sources.len() {
                                            sources.swap(index, index - 1);
                                        }
                                    },
                                    "Up"
                                }
                                button {
                                    r#type: "button",
                                    aria_label: "Move source {index + 1} down",
                                    disabled: unavailable || index + 1 == paths.len(),
                                    onclick: move |_| {
                                        let mut sources = inputs.write();
                                        if index + 1 < sources.len() {
                                            sources.swap(index, index + 1);
                                        }
                                    },
                                    "Down"
                                }
                                button {
                                    r#type: "button",
                                    aria_label: "Remove source {index + 1}",
                                    disabled: unavailable,
                                    onclick: move |_| {
                                        let mut sources = inputs.write();
                                        if index < sources.len() {
                                            sources.remove(index);
                                        }
                                    },
                                    "Remove"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "job-status", role: "status", aria_live: "polite", "{status}" }
        }
    }
}

async fn run_merge(
    queue: &JobQueue,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    mut status: Signal<String>,
) -> String {
    let mut events = match start_pdf_merge(queue, inputs, output.clone()) {
        Ok(events) => events,
        Err(error) => return format!("Merge failed. {error}"),
    };
    while let Some(event) = events.recv().await {
        match event {
            JobEvent::Processing => status.set("Merging PDF pages…".into()),
            JobEvent::Finished(Ok(Ok(pages))) => {
                return format!("Saved {pages} pages to {}.", output.display());
            }
            JobEvent::Finished(Ok(Err(error))) => return format!("Merge failed. {error}"),
            JobEvent::Finished(Err(error)) => return format!("Merge failed. {error}"),
        }
    }
    "Merge failed. The job ended without a result.".into()
}
