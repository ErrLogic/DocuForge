use std::path::Path;

use docuforge_core::{JobEvent, JobQueue, StartMergeError, start_pdf_merge};
use lopdf::{Document, Object, dictionary};
use tokio::runtime::Builder;

fn write_pdf(path: &Path, width: i64) {
    let mut pdf = Document::with_version("1.7");
    let pages = pdf.new_object_id();
    let page = pdf.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages,
        "MediaBox" => vec![0.into(), 0.into(), width.into(), 200.into()],
        "Resources" => dictionary! {},
    });
    pdf.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Count" => 1, "Kids" => vec![Object::Reference(page)],
        }),
    );
    let catalog = pdf.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    pdf.trailer.set("Root", catalog);
    pdf.save(path).unwrap();
}

#[test]
fn merges_through_the_background_job_and_reports_failure_on_retry() {
    let directory = tempfile::tempdir().unwrap();
    let first = directory.path().join("first.pdf");
    let second = directory.path().join("second.pdf");
    let output = directory.path().join("merged.pdf");
    write_pdf(&first, 200);
    write_pdf(&second, 300);

    let queue = JobQueue::default();
    assert!(matches!(
        start_pdf_merge(&queue, vec![first.clone()], output.clone()),
        Err(StartMergeError::Invalid(_))
    ));

    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let sources = vec![second, first];
            let mut events = start_pdf_merge(&queue, sources.clone(), output.clone()).unwrap();
            assert!(matches!(events.recv().await, Some(JobEvent::Processing)));
            assert!(matches!(
                events.recv().await,
                Some(JobEvent::Finished(Ok(Ok(2))))
            ));
            assert!(events.recv().await.is_none());

            let bytes = std::fs::read(&output).unwrap();
            let pdf = Document::load_mem(&bytes).unwrap();
            let widths: Vec<_> = pdf
                .page_iter()
                .map(|id| {
                    pdf.get_dictionary(id)
                        .unwrap()
                        .get(b"MediaBox")
                        .unwrap()
                        .as_array()
                        .unwrap()[2]
                        .as_i64()
                        .unwrap()
                })
                .collect();
            assert_eq!(widths, [300, 200]);

            let mut retry = start_pdf_merge(&queue, sources, output.clone()).unwrap();
            assert!(matches!(retry.recv().await, Some(JobEvent::Processing)));
            assert!(matches!(
                retry.recv().await,
                Some(JobEvent::Finished(Ok(Err(_))))
            ));
            assert_eq!(std::fs::read(output).unwrap(), bytes);
        });
}
