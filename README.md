# DocuForge

DocuForge is an offline-first desktop toolkit for PDF, image, and document processing.
The desktop workspace can merge PDFs and switch between dark and light themes. The default theme is dark.

## Development

Install Rust through rustup. The workspace uses stable Rust, rustfmt, and Clippy.

- Windows requires the WebView2 Runtime.
- macOS uses the system WebKit runtime and requires the Xcode Command Line Tools for building.
- On Debian and Ubuntu, install the native desktop build dependencies:

  ```sh
  sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev libxdo-dev
  ```

On Linux, the file dialogs use the desktop XDG portal. Install the portal backend for your desktop session. RFD can fall back to Zenity when the portal is unavailable.

See the [Dioxus platform setup guide](https://dioxuslabs.com/learn/0.7/getting_started/) for other Linux distributions.

Run the shell from the repository root:

```sh
cargo run --locked -p docuforge
```

Validate the workspace:

```sh
cargo fmt --all -- --check
cargo check --locked --workspace
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace
```

## Test milestones independently

### Desktop shell

Run `cargo run --locked -p docuforge` with the desktop dependencies installed.
Confirm that the DocuForge window opens in dark mode and that **Light theme**
switches between the two themes.

### PDF merge in the desktop app

1. Click **Add PDFs** and select at least two files.
2. Use **Up**, **Down**, or **Remove** to arrange the source list.
3. Click **Merge and save** and choose a new destination. The status reports
   when processing starts and where the result was saved.
4. Open the resulting PDF and check that its page order matches the list.
   Try the same destination again to check that the app reports the conflict
   and preserves the first output.

To verify the background job and its generated PDF without opening a desktop
window, run:

```sh
cargo test --locked -p docuforge-core -p docuforge-jobs
```

### Standalone PDF merge command

Build and test the standalone merge command with Rust and Cargo. It does not
require a desktop WebView, PDFium, or LibreOffice.

```sh
cargo test --locked -p docuforge-pdf
cargo build --locked -p docuforge-pdf --bin docuforge-merge
cargo run --locked -p docuforge-pdf --bin docuforge-merge -- merged.pdf "first.pdf" "second.pdf"
```

Use two existing PDFs and a new output path. Open the result in a PDF viewer
and check that all pages from the first input appear before those from the
second. Repeating the command with the same output path should fail and leave
the first result intact. This milestone has a 128 MiB combined input limit.
Encrypted PDFs and interactive forms are rejected. The result carries page
content and resources, while document-level metadata and bookmarks are omitted.

### GitHub Actions binaries

For a successful CI run, download the artifact matching your OS and CPU
architecture from the run's **Artifacts** section. Extract it and run the
`docuforge` and `docuforge-merge` binaries (`.exe` on Windows). On macOS and
Linux, restore executable permission after extraction with
`chmod +x docuforge docuforge-merge`. The desktop binary still requires its
system WebView runtime.
