use std::{fs, path::Path, process::Command};

use lopdf::{Document, Object, Stream, dictionary};

fn fixture(path: &Path, width: i64, content: &[u8]) {
    let mut pdf = Document::with_version("1.7");
    let pages = pdf.new_object_id();
    let stream = pdf.add_object(Stream::new(dictionary! {}, content.to_vec()));
    let page = pdf.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => stream,
    });
    pdf.objects.insert(
        pages,
        Object::Dictionary(dictionary! {
            "Type" => "Pages", "Count" => 1, "Kids" => vec![Object::Reference(page)],
            "MediaBox" => vec![0.into(), 0.into(), width.into(), 200.into()],
            "Resources" => dictionary! {},
        }),
    );
    let catalog = pdf.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    pdf.trailer.set("Root", catalog);
    pdf.save(path).unwrap();
}

#[test]
fn merges_in_argument_order_and_preserves_inherited_page_dimensions() {
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("first input.pdf");
    let second = dir.path().join("second.pdf");
    let output = dir.path().join("merged.pdf");
    fixture(&first, 250, b"1 0 0 rg");
    fixture(&second, 400, b"0 1 0 rg");

    let process = Command::new(env!("CARGO_BIN_EXE_docuforge-merge"))
        .args([&output, &second, &first])
        .output()
        .unwrap();
    assert!(process.status.success(), "{:?}", process.stderr);
    assert_eq!(process.stdout.trim_ascii(), b"Wrote 2 pages.");

    let pdf = Document::load(output).unwrap();
    let pages = pdf.get_pages();
    assert_eq!(pages.len(), 2);
    for (index, width, content) in [(1, 400, &b"0 1 0 rg"[..]), (2, 250, &b"1 0 0 rg"[..])] {
        let page = pages[&index];
        assert_eq!(pdf.get_page_content(page).trim_ascii(), content);
        let parent = pdf
            .get_dictionary(page)
            .unwrap()
            .get(b"Parent")
            .unwrap()
            .as_reference()
            .unwrap();
        let media_box = pdf
            .get_dictionary(parent)
            .unwrap()
            .get(b"MediaBox")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(media_box[2].as_i64().unwrap(), width);
    }
}

#[test]
fn preserves_existing_outputs_and_leaves_no_output_for_invalid_input() {
    let dir = tempfile::tempdir().unwrap();
    let valid = dir.path().join("original.pdf");
    let invalid = dir.path().join("invalid.pdf");
    let output = dir.path().join("output.pdf");
    fixture(&valid, 250, b"1 0 0 rg");
    let original = fs::read(&valid).unwrap();

    assert!(docuforge_pdf::merge(&[valid.clone(), valid.clone()], &valid).is_err());
    assert_eq!(fs::read(&valid).unwrap(), original);

    for contents in [b"".as_slice(), b"not a pdf", b"%PDF-1.7\nmalformed"] {
        fs::write(&invalid, contents).unwrap();
        assert!(docuforge_pdf::merge(&[valid.clone(), invalid.clone()], &output).is_err());
        assert!(!output.exists());
    }
}
