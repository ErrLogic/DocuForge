//! Local PDF processing.

use std::{
    fs::{self, File},
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

use lopdf::{Document, Object, dictionary};
use tempfile::NamedTempFile;

/// An error encountered while merging local PDFs.
#[derive(Debug, thiserror::Error)]
pub enum MergeError {
    /// At least two source documents are required.
    #[error("select at least two source PDFs")]
    TooFewInputs,
    /// An input could not be loaded or validated.
    #[error("invalid PDF {path:?}: {reason}")]
    Input {
        /// The source file.
        path: PathBuf,
        /// The validation failure.
        reason: String,
    },
    /// The destination could not be written or already exists.
    #[error("cannot write {path:?}: {source}")]
    Output {
        /// The requested destination.
        path: PathBuf,
        /// The underlying I/O failure.
        source: io::Error,
    },
}

const MAX_TOTAL_INPUT_BYTES: u64 = 128 * 1024 * 1024;

/// Merge source PDFs in argument order and return the number of pages written.
///
/// Creates a new destination atomically. An existing destination is left intact.
/// The merged document carries page content and page resources. Document-level
/// metadata and bookmarks are omitted, and interactive forms are rejected.
pub fn merge(inputs: &[PathBuf], output: &Path) -> Result<usize, MergeError> {
    if inputs.len() < 2 {
        return Err(MergeError::TooFewInputs);
    }

    let mut merged = Document::with_version("2.0");
    let root = merged.new_object_id();
    let mut children = Vec::with_capacity(inputs.len());
    let mut total_pages = 0;
    let mut remaining = MAX_TOTAL_INPUT_BYTES;

    for path in inputs {
        let invalid = |reason| MergeError::Input {
            path: path.clone(),
            reason,
        };
        let mut source = load_source(path, &mut remaining).map_err(invalid)?;
        let count = source.page_iter().count();
        if count == 0 {
            return Err(invalid("the document contains no readable pages".into()));
        }

        source.renumber_objects_with(merged.max_id + 1);
        let source_root = source
            .catalog()
            .and_then(|catalog| catalog.get(b"Pages"))
            .and_then(Object::as_reference)
            .map_err(|error| invalid(error.to_string()))?;

        // Keep each source page tree intact so its inherited resources, rotation,
        // and page dimensions remain scoped to the original pages.
        let pages = source
            .get_object_mut(source_root)
            .and_then(Object::as_dict_mut)
            .map_err(|error| invalid(error.to_string()))?;
        let declared_count = pages
            .get(b"Count")
            .and_then(Object::as_i64)
            .map_err(|error| invalid(error.to_string()))?;
        if declared_count != count as i64 {
            return Err(invalid(
                "page tree count does not match readable pages".into(),
            ));
        }
        pages.set("Parent", root);

        total_pages += count;
        children.push(Object::Reference(source_root));
        merged.max_id = source.max_id;
        merged.objects.append(&mut source.objects);
    }

    merged.objects.insert(
        root,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => children,
            "Count" => total_pages as i64,
        }),
    );
    let catalog = merged.add_object(dictionary! { "Type" => "Catalog", "Pages" => root });
    merged.trailer.set("Root", catalog);
    merged.prune_objects();

    let write_error = |source| MergeError::Output {
        path: output.to_path_buf(),
        source,
    };
    let directory = output.parent().filter(|path| !path.as_os_str().is_empty());
    let mut temporary =
        NamedTempFile::new_in(directory.unwrap_or_else(|| Path::new("."))).map_err(write_error)?;
    merged
        .save_to(temporary.as_file_mut())
        .map_err(write_error)?;
    temporary.as_file().sync_all().map_err(write_error)?;
    temporary
        .persist_noclobber(output)
        .map_err(|error| write_error(error.error))?;

    Ok(total_pages)
}

fn load_source(path: &Path, remaining: &mut u64) -> Result<Document, String> {
    let io_error = |error: io::Error| error.to_string();
    let metadata = fs::symlink_metadata(path).map_err(io_error)?;
    if !metadata.is_file() {
        return Err("expected a regular file".into());
    }
    if metadata.len() > *remaining {
        return Err("combined source size exceeds 128 MiB".into());
    }

    let mut file = File::open(path).map_err(io_error)?;
    let mut header = [0; 5];
    file.read_exact(&mut header).map_err(io_error)?;
    if &header != b"%PDF-" {
        return Err("missing PDF signature".into());
    }
    file.rewind().map_err(io_error)?;

    let mut bytes = Vec::new();
    file.take(*remaining + 1)
        .read_to_end(&mut bytes)
        .map_err(io_error)?;
    if bytes.len() as u64 > *remaining {
        return Err("combined source size exceeds 128 MiB".into());
    }
    *remaining -= bytes.len() as u64;

    let encrypted = "encrypted PDFs are unsupported";
    let source = Document::load_mem(&bytes).map_err(|error| match error {
        lopdf::Error::InvalidPassword => encrypted.into(),
        other => other.to_string(),
    })?;
    if source.was_encrypted() || source.trailer.has(b"Encrypt") {
        return Err(encrypted.into());
    }
    if source
        .catalog()
        .map_err(|error| error.to_string())?
        .has(b"AcroForm")
    {
        return Err("PDFs with interactive forms are unsupported".into());
    }
    Ok(source)
}
