# AGENTS.md

# DocuForge — AI Agent Engineering Guidelines

> **Project:** DocuForge  
> **Description:** Offline-first desktop document and image processing toolkit.  
> **Architecture:** Rust workspace with Dioxus Desktop frontend.  
> **Target Platforms:** Windows, macOS, Linux.  
> **Distribution:** Standalone installers.  
> **Project Status:** Initial development.

---

## 1. Project Overview

DocuForge is a cross-platform, offline-first desktop application for document and image processing.

The application provides a unified workspace for manipulating PDF files, converting images, and converting documents into PDF format.

All file processing must happen locally on the user's machine.

### Core Features

#### PDF Tools
- PDF compression.
- PDF splitting.
- PDF merging.
- PDF page extraction.
- PDF page selection.
- PDF to image conversion.
- PDF preview.

#### Image Tools
- Image format conversion.
- Image to PDF conversion.
- Batch image conversion.
- Image resizing.
- Image quality adjustment.
- Image metadata inspection.

#### Document Tools
- Markdown to PDF.
- TXT to PDF.
- DOC to PDF.
- DOCX to PDF.
- ODT to PDF.
- Additional document formats where supported.

### Product Principles

1. Offline-first: No cloud dependency for core functionality.
2. Privacy-first: User files must remain on the local machine.
3. Performance: Heavy processing must never block the UI.
4. Reliability: Original files must never be modified unintentionally.
5. Maintainability: Processing engines must remain independent of the UI.
6. Cross-platform: Windows, macOS, and Linux must be considered from the beginning.
7. Extensibility: New processing engines and formats must be easy to integrate.

---

## 2. Technology Stack

Use the following stack unless an explicit architectural decision changes it.

| Layer | Technology |
|---|---|
| Programming language | Rust |
| Desktop framework | Dioxus Desktop |
| Dioxus version | 0.7.x baseline |
| UI rendering | Native Rust + WebView |
| UI markup | Dioxus RSX |
| Styling | CSS / CSS Variables |
| Async runtime | Tokio |
| Parallel processing | Rayon |
| Serialization | Serde |
| Error handling | thiserror + anyhow |
| Logging | tracing |
| PDF manipulation | lopdf |
| PDF rendering | pdfium-render |
| Image processing | image |
| SVG rendering | resvg |
| Markdown parsing | pulldown-cmark |
| Document conversion | LibreOffice headless adapter |
| Package management | Cargo |
| Build tooling | Dioxus CLI |

### Important Constraints

- Do not introduce React, Vue, Svelte, Angular, or other JavaScript frontend frameworks.
- Do not introduce Tauri into the architecture.
- Do not use Node.js as an application runtime.
- Do not move business logic into the frontend.
- Do not introduce unnecessary external services.
- Prefer pure Rust dependencies where practical.
- Native dependencies must be isolated behind adapters.

Dioxus Desktop uses a WebView runtime. Do not assume that all operating systems provide an identical WebView environment.

---

## 3. Architecture

DocuForge follows a modular architecture inspired by Hexagonal Architecture (Ports and Adapters).

The application must maintain strict boundaries between:

1. Presentation Layer
2. Application Layer
3. Domain Layer
4. Infrastructure Layer

### Architectural Rules

#### Presentation Layer

Location: `apps/desktop/`

Responsibilities:
- Render the application UI.
- Handle user interactions.
- Display processing progress.
- Manage presentation state.
- Invoke application services.
- Display errors and notifications.

The presentation layer must not contain PDF or image processing algorithms.

#### Application Layer

Location: `crates/docuforge-core/`

Responsibilities:
- Coordinate application use cases.
- Define application services.
- Coordinate processing engines.
- Validate operation requests.
- Manage application-level workflows.

#### Domain Layer

Location: `crates/docuforge-domain/`

Responsibilities:
- Define core business models.
- Define processing operations.
- Define job states.
- Define shared domain errors.
- Define engine interfaces where appropriate.

Domain models must not depend on Dioxus, WebView, or native UI frameworks.

#### Infrastructure Layer

Locations:
- `crates/docuforge-pdf/`
- `crates/docuforge-image/`
- `crates/docuforge-document/`
- `crates/docuforge-filesystem/`
- `crates/docuforge-jobs/`

Responsibilities:
- Implement processing engines.
- Interact with the local filesystem.
- Manage temporary files.
- Execute background jobs.
- Integrate external native dependencies.

Infrastructure implementations must not depend on presentation components.

---

## 4. Repository Structure

Maintain the following workspace organization:

```text
docuforge/
├── apps/
│   └── desktop/
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs
│       │   ├── routes/
│       │   ├── pages/
│       │   ├── components/
│       │   │   ├── common/
│       │   │   ├── layout/
│       │   │   ├── pdf/
│       │   │   └── image/
│       │   ├── state/
│       │   └── services/
│       ├── assets/
│       ├── styles/
│       ├── Cargo.toml
│       └── Dioxus.toml
│
├── crates/
│   ├── docuforge-core/
│   ├── docuforge-domain/
│   ├── docuforge-jobs/
│   ├── docuforge-filesystem/
│   ├── docuforge-pdf/
│   ├── docuforge-image/
│   └── docuforge-document/
│
├── Cargo.toml
├── rust-toolchain.toml
├── .gitignore
├── README.md
└── AGENTS.md
```

Do not create unnecessary modules or directories without a clear architectural reason.

Each crate must have a clearly defined responsibility.

---

## 5. Coding Standards

### Rust

- Use Rust 2024 edition unless compatibility requirements dictate otherwise.
- Prefer stable Rust.
- Avoid unsafe Rust unless absolutely necessary.
- Document every necessary unsafe block.
- Prefer ownership and borrowing over unnecessary cloning.
- Use explicit types where they improve readability.
- Avoid excessive generic abstractions.
- Avoid unnecessary trait hierarchies.
- Prefer composition over inheritance-like patterns.
- Keep functions focused and reasonably small.
- Use meaningful names.
- Avoid unexplained magic numbers.
- Use `Result<T, E>` for recoverable failures.
- Avoid panics in normal application execution.

### Error Handling

Use:
- `thiserror` for structured errors in libraries.
- `anyhow` for application-level error propagation where appropriate.

Errors must contain useful context.

Do not silently ignore errors.

Do not use `unwrap()` or `expect()` in production processing paths unless an invariant is explicitly documented and guaranteed.

### Documentation

- Public APIs must have meaningful Rust documentation.
- Document non-obvious algorithms.
- Document platform-specific behavior.
- Document external native dependency requirements.
- Keep comments focused on why something is implemented, not merely restating what the code does.

### Dependency Management

- Centralize shared workspace dependencies in the root `Cargo.toml`.
- Avoid unnecessary dependencies.
- Prefer mature and maintained crates.
- Evaluate licensing before introducing dependencies.
- Do not blindly upgrade dependencies.
- Commit `Cargo.lock` for reproducible application builds.

---

## 6. Frontend Guidelines

The frontend must use Dioxus and Rust.

### Component Architecture

Build reusable components.

Examples:

- `AppSidebar`
- `AppHeader`
- `DropZone`
- `FileCard`
- `FilePreview`
- `JobProgress`
- `JobQueue`
- `Toast`
- `Dialog`
- `CommandPalette`

Keep page components focused on page composition.

Do not place all UI logic inside a single root component.

### Design System

DocuForge uses a modern, minimalist desktop design inspired by productivity applications such as Linear and Raycast.

Design principles:

- Dark mode as the default theme.
- Light mode as an alternative.
- Consistent spacing.
- Consistent typography.
- Restrained use of colors.
- Subtle borders and shadows.
- Moderate corner radius.
- Smooth but restrained transitions.
- Clear visual hierarchy.
- Accessible contrast.

Use CSS variables for design tokens.

Example:

```css
:root {
    --color-background: #111318;
    --color-surface: #191c23;
    --color-surface-hover: #222631;
    --color-border: #303542;
    --color-text-primary: #f4f5f7;
    --color-text-secondary: #a1a6b3;
    --color-accent: #8172ed;

    --radius-sm: 6px;
    --radius-md: 10px;
    --radius-lg: 14px;
}
```

Do not hardcode design values repeatedly across components.

Avoid excessive animations that interfere with responsiveness.

### UI Performance

- Avoid unnecessary component re-rendering.
- Keep large file lists efficient.
- Use virtualization when lists become large.
- Do not load entire image files into UI state unnecessarily.
- Generate previews asynchronously.
- Never execute heavy file processing directly in UI event handlers.

---

## 7. Job Processing Architecture

All potentially expensive operations must execute through the job processing system.

Examples:
- PDF compression.
- PDF rendering.
- Image conversion.
- Batch image processing.
- Document conversion.

### Job Lifecycle

```text
Pending
   |
   v
Validating
   |
   v
Queued
   |
   v
Processing
   |
   +----> Completed
   |
   +----> Failed
   |
   +----> Cancelled
```

### Job Requirements

Each job should have:

- Unique identifier.
- Operation type.
- Input files.
- Output destination.
- Processing status.
- Progress information.
- Creation timestamp.
- Completion timestamp where applicable.
- Error information where applicable.

### Concurrency

- Use Tokio for asynchronous orchestration.
- Use Rayon for suitable CPU-intensive parallel operations.
- Do not blindly run CPU-intensive work on Tokio's async executor.
- Use bounded concurrency.
- Avoid unbounded task spawning.
- Implement cancellation where practical.
- Ensure temporary resources are cleaned up after completion or cancellation.

### Progress Reporting

Progress must be communicated to the frontend through a consistent event or subscription mechanism.

Do not implement aggressive polling loops.

Progress should be meaningful and operation-specific.

For example:

```text
PDF compression:
- Analyzing document
- Optimizing streams
- Processing images
- Writing output
- Completed
```

---

## 8. PDF Engine Guidelines

Location: `crates/docuforge-pdf/`

### Responsibilities

- Merge PDF files.
- Split PDF files.
- Extract pages.
- Compress PDF files.
- Render PDF pages.
- Generate PDF previews.
- Export PDF pages as images.

### Engine Responsibilities

Use `lopdf` for PDF structural manipulation.

Use `pdfium-render` for PDF rendering where appropriate.

Keep these implementations behind stable application-facing interfaces.

### Compression

Support at least two conceptual modes:

1. Lossless optimization.
2. Lossy optimization.

Lossy optimization must expose quality-related settings.

Never promise that every PDF can be compressed to a smaller size.

### Safety

- Never overwrite source files by default.
- Validate PDF input.
- Handle encrypted PDFs explicitly.
- Handle malformed documents gracefully.
- Preserve page ordering during merge and split.
- Report unsupported PDF features clearly.

---

## 9. Image Engine Guidelines

Location: `crates/docuforge-image/`

### Responsibilities

- Decode supported image formats.
- Encode supported image formats.
- Convert image formats.
- Resize images.
- Handle image quality settings.
- Generate image previews.
- Inspect image metadata.

### Format Handling

Do not assume that MIME type guarantees format compatibility.

Determine the actual file format using decoder detection and file signatures.

Maintain an explicit list of supported decoder and encoder formats.

### Conversion Considerations

Handle:

- Transparency.
- Color profiles.
- Image orientation.
- Animation.
- Metadata preservation.
- Quality settings.
- Memory usage.

Examples:

- PNG to JPEG requires an explicit background policy for transparent pixels.
- SVG to JPEG requires rasterization.
- Animated GIF conversion must define whether animation is preserved or only the first frame is processed.
- Large images must not cause uncontrolled memory allocation.

Never claim support for a format until its actual decoder and encoder have been tested.

---

## 10. Document Engine Guidelines

Location: `crates/docuforge-document/`

### Responsibilities

Convert supported document formats into PDF.

### Conversion Strategy

Use internal Rust implementations where practical.

Examples:

- Markdown: `pulldown-cmark`.
- TXT: Internal HTML generation.
- HTML: A suitable rendering adapter.

Use LibreOffice headless as an external conversion adapter for office formats where necessary.

### Requirements

- Isolate external process execution.
- Never construct shell commands using untrusted user input.
- Use argument-based process invocation.
- Apply execution timeouts.
- Capture process output.
- Detect missing executables.
- Report conversion failures clearly.
- Validate generated PDF output.

Do not attempt to implement a complete DOCX layout engine from scratch.

---

## 11. Filesystem and Security

Location: `crates/docuforge-filesystem/`

File handling is a critical security boundary.

### Mandatory Rules

- Validate file signatures.
- Do not trust file extensions.
- Do not trust user-provided MIME types.
- Avoid overwriting original files.
- Use safe temporary directories.
- Clean up temporary files.
- Handle filesystem permissions.
- Handle path traversal risks.
- Avoid following unexpected symbolic links.
- Apply file size and resource limits.
- Avoid uncontrolled memory allocations.
- Treat malformed files as untrusted input.

### External Processes

When invoking LibreOffice or other external tools:

- Never use shell interpolation.
- Pass arguments separately.
- Use controlled environment variables.
- Apply timeouts.
- Capture stderr and stdout.
- Validate output files.
- Handle process termination correctly.

### Privacy

- No automatic file uploads.
- No remote document processing.
- No telemetry by default.
- No automatic dependency downloads during file processing.
- Do not log document contents.
- Do not log sensitive file metadata unnecessarily.

---

## 12. Cross-Platform Requirements

Target platforms:

- Windows 10/11.
- macOS.
- Linux.

Do not assume identical native dependency behavior across operating systems.

### Platform Considerations

Windows:
- WebView2 runtime.
- Native DLL packaging.
- Windows path semantics.
- Installer integration.

macOS:
- WebKit runtime.
- App bundle structure.
- Code signing and notarization.
- Intel and Apple Silicon compatibility.

Linux:
- WebKitGTK dependencies.
- Distribution-specific libraries.
- DEB/RPM/AppImage packaging where supported.

### Architecture

Test supported CPU architectures explicitly.

Do not assume an x86_64 build is compatible with ARM64.

Standalone distribution must account for all required native runtime dependencies.

Optional system dependencies must be detected and reported clearly.

---

## 13. Testing Strategy

Testing is mandatory for all processing engines.

### Unit Tests

Test:
- Domain models.
- Validation logic.
- Conversion settings.
- Error handling.
- Job lifecycle.
- File type detection.

### Integration Tests

Test:
- PDF merge.
- PDF split.
- PDF compression.
- Image conversion.
- Image to PDF.
- PDF to image.
- Document to PDF.

### Cross-Platform Tests

Run CI on Windows, macOS, and Linux.

### Test Requirements

- Use temporary directories.
- Do not modify test fixtures.
- Test malformed files.
- Test empty files.
- Test unsupported formats.
- Test large inputs within defined resource limits.
- Test cancellation.
- Test error propagation.

For image and PDF processing, verify the generated output rather than checking only whether the operation returned `Ok(())`.

---

## 14. Build and Development Commands

Expected development commands:

```bash
# Check workspace
cargo check --workspace

# Run unit tests
cargo test --workspace

# Run linting
cargo clippy --workspace --all-targets --all-features

# Format code
cargo fmt --all

# Run desktop application
cd apps/desktop
dx serve --platform desktop

# Build desktop application
dx build --platform desktop --release
```

Verify commands against the actual installed Dioxus CLI version before changing CI or documenting platform-specific build procedures.

---

## 15. GitHub Actions & CI/CD

GitHub Actions is the official CI/CD platform for DocuForge.

All automated builds and validation workflows must run through GitHub Actions.

### Branch Strategy

- `main` is the primary development and integration branch.
- All production-ready changes must be integrated into `main`.
- Pull requests targeting `main` must pass required CI checks.
- Direct commits to `main` should be avoided when branch protection is available.
- Do not create or assume additional permanent branches without an explicit requirement.

### Continuous Integration

Every pull request targeting `main` and every push to `main` must trigger CI.

CI must include:

1. Rust formatting validation.
2. Workspace compilation checks.
3. Clippy linting.
4. Unit tests.
5. Integration tests where applicable.
6. Platform-specific build validation.

### Cross-Platform Build Matrix

GitHub Actions must build and validate the application on:

| Platform | Runner |
|---|---|
| Windows | `windows-latest` |
| macOS | `macos-latest` |
| Linux | `ubuntu-latest` |

Use explicit target architectures where required.

Do not assume that a successful build on one operating system guarantees compatibility with other platforms.

### Build Triggers

The CI/CD workflow must support:

- `push` to `main`.
- `pull_request` targeting `main`.
- Version tags matching `v*` for release builds.
- Manual workflow dispatch when appropriate.

### Build Artifacts

Successful release builds must produce platform-appropriate distributable artifacts.

Expected formats include:

- Windows: MSI / EXE.
- macOS: DMG / APP bundle.
- Linux: DEB / RPM / AppImage where supported.

Build artifacts must be uploaded to GitHub Actions artifacts.

### Release Strategy

Official releases must be triggered by version tags.

Examples:

```bash
git tag v0.1.0
git push origin v0.1.0
```

A version tag must trigger:

1. Version validation.
2. Cross-platform release builds.
3. Artifact validation.
4. GitHub Release creation.
5. Release artifact publication.

Never publish a release from an unverified or failed build.

### Native Dependency Packaging

GitHub Actions workflows must account for platform-specific native dependencies.

This includes:

- WebView runtime requirements.
- PDFium binaries.
- LibreOffice distribution.
- Native image processing libraries, if introduced.

Do not assume native dependencies are available on GitHub-hosted runners.

Bundle required dependencies or explicitly validate their availability.

### Workflow Security

- Use GitHub Actions permissions with least privilege.
- Pin third-party Actions to reviewed versions or commit SHAs.
- Never expose secrets in build logs.
- Do not execute untrusted pull request code with privileged secrets.
- Do not store signing certificates or private keys in the repository.
- Use GitHub Secrets for signing credentials when signing is configured.
- Separate CI validation permissions from release publishing permissions.

### AI Agent Instructions

When modifying GitHub Actions workflows:

1. Inspect existing workflows before making changes.
2. Preserve existing build and release behavior unless explicitly requested otherwise.
3. Keep workflows modular and maintainable.
4. Avoid unnecessary duplicated jobs.
5. Validate YAML syntax and workflow configuration.
6. Document newly introduced build dependencies.
7. Never claim a GitHub Actions workflow passed unless it was actually executed successfully.

---

## 16. Architectural Decisions

The following decisions are considered established project constraints.

| Decision | Status |
|---|---|
| Rust as primary language | Accepted |
| Dioxus Desktop frontend | Accepted |
| No Tauri | Accepted |
| No JavaScript frontend framework | Accepted |
| Workspace-based architecture | Accepted |
| Offline-first processing | Accepted |
| Standalone distribution | Accepted |
| Windows, macOS, Linux targets | Accepted |
| Custom CSS design system | Accepted |
| Background job processing | Accepted |
| Native dependency adapters | Accepted |
| GitHub Actions CI/CD | Accepted |
| `main` as primary branch | Accepted |
| Tag-based release builds | Accepted |

Any change to these decisions must include a clear technical justification.

---

## 17. Development Philosophy

DocuForge should be built as a maintainable software product, not a collection of unrelated conversion scripts.

Prioritize:

1. Correctness over feature count.
2. Reliability over shortcuts.
3. Explicit contracts over hidden coupling.
4. Resource safety over uncontrolled concurrency.
5. Clear error reporting over silent failure.
6. Reproducible builds over environment-dependent behavior.
7. Testable architecture over premature abstraction.

Do not optimize code prematurely.

Do not introduce complex abstractions without a demonstrated need.

Keep the application simple where possible and rigorous where necessary.

---

## 18. Git Identity and Attribution

- Use this repository's configured `user.name` and `user.email` for Git commits. Do not override them through flags, environment variables, or configuration changes.
- Push through the repository's configured remote using the user's existing Git authentication.
- Keep commit subjects, bodies, trailers, author identities, and committer identities free of AI attribution, including AI co-author and generator credits.

---

**End of AGENTS.md**
