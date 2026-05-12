use crate::api::projects::{
    FluentDirectoryDto, FluentFileDto, ProjectDto, ProjectScanDto, fetch_project,
    finish_onboarding, scan_project_fluent_files,
};
use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, MetaFor, RenderIcon, Skeleton,
};
use dioxus::prelude::*;

#[component]
pub fn OnboardingProject(project_id: String) -> Element {
    let project_id_for_fetch = project_id.clone();

    let project = use_resource(move || {
        let project_id = project_id_for_fetch.clone();

        async move { fetch_project(project_id).await.ok().flatten() }
    });

    let mut scan_result = use_signal(|| None::<ServerFnResult<ProjectScanDto>>);
    let mut scanning = use_signal(|| false);

    rsx! {
        crate::components::Meta { r#for: MetaFor::Home }

        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl flex-col justify-center px-6 py-24 lg:px-10",

            match project.read().as_ref() {
                None => rsx! {
                    ProjectSkeleton {}
                },

                Some(None) => rsx! {
                    ProjectNotFound {}
                },

                Some(Some(project)) => rsx! {
                    div {
                        class: "grid w-full items-start gap-12 lg:grid-cols-[0.85fr_1.15fr]",

                        ProjectIntro {
                            project: project.clone(),
                        }

                        ScanCard {
                            project_id: project_id.clone(),
                            scan_result,
                            scanning,
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn ProjectSkeleton() -> Element {
    rsx! {
        div {
            class: "grid w-full items-start gap-12 lg:grid-cols-[0.85fr_1.15fr]",

            Skeleton {
                suspend: true,
                class: "h-80 w-full rounded-3xl border border-white/[0.08] bg-white/[0.03]".to_string(),
                div {}
            }

            Skeleton {
                suspend: true,
                class: "h-96 w-full rounded-3xl border border-white/[0.08] bg-white/[0.03]".to_string(),
                div {}
            }
        }
    }
}

#[component]
fn ProjectNotFound() -> Element {
    rsx! {
        div {
            class: "max-w-3xl",

            h1 {
                class: "font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)]",
                "Project not found."
            }

            p {
                class: "mt-6 text-lg leading-8 text-[color:var(--text-secondary)]",
                "This project does not exist or you do not have access to it."
            }
        }
    }
}

#[component]
fn ProjectIntro(project: ProjectDto) -> Element {
    rsx! {
        div {
            class: "max-w-3xl",

            h1 {
                class: "mt-6 font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-6xl lg:text-7xl",
                "Set up your"
                br {}
                "translations"
                span {
                    class: "text-[color:var(--bg-accent)]",
                    "."
                }
            }

            p {
                class: "mt-7 max-w-xl text-lg leading-8 text-[color:var(--text-secondary)]",
                "We imported {project.repository_full_name}. Now choose the directory BiteFluent should treat as your translation source."
            }

            div {
                class: "mt-8 rounded-2xl border border-white/[0.08] bg-white/[0.03] p-5",

                div {
                    class: "flex items-center gap-4",

                    div {
                        class: "flex size-11 shrink-0 items-center justify-center rounded-2xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/10 text-[color:var(--bg-accent)]",

                        RenderIcon {
                            kind: IconKind::Repository,
                            class: Some("size-5".to_string()),
                        }
                    }

                    div {
                        class: "min-w-0",

                        p {
                            class: "truncate text-sm font-semibold text-[color:var(--text-primary)]",
                            "{project.repository_full_name}"
                        }

                        p {
                            class: "mt-1 text-sm text-[color:var(--text-muted)]",
                            "Default branch: {project.default_branch}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScanCard(
    project_id: String,
    scan_result: Signal<Option<ServerFnResult<ProjectScanDto>>>,
    scanning: Signal<bool>,
) -> Element {
    let navigator = use_navigator();

    let mut selected_directory = use_signal(|| None::<String>);
    let mut finishing = use_signal(|| false);
    let mut finish_error = use_signal(|| None::<String>);

    let scan_state = scan_result.read();
    let has_successful_scan = matches!(scan_state.as_ref(), Some(Ok(_)));

    let start_scan = move |project_id: String,
                           mut scanning: Signal<bool>,
                           mut scan_result: Signal<Option<ServerFnResult<ProjectScanDto>>>,
                           mut selected_directory: Signal<Option<String>>| {
        scanning.set(true);
        scan_result.set(None);
        selected_directory.set(None);

        spawn(async move {
            let result = scan_project_fluent_files(project_id).await;

            if let Ok(scan) = result.as_ref() {
                let directory = scan.suggested_locales_path.clone().or_else(|| {
                    scan.directories
                        .first()
                        .map(|directory| directory.path.clone())
                });

                selected_directory.set(directory);
            }

            scanning.set(false);
            scan_result.set(Some(result));
        });
    };

    rsx! {
        div {
            class: "flex w-full max-w-2xl flex-col justify-self-end rounded-3xl border border-white/[0.10] bg-white/[0.035] p-6 shadow-2xl shadow-black/30 backdrop-blur sm:p-8 lg:max-h-[720px] lg:overflow-hidden",

            ScanCardHeader {}

            div {
                class: "mt-6 min-h-0 flex-1 lg:overflow-y-auto lg:pr-1",

                if scanning() {
                    ScanSkeleton {}
                } else {
                    match scan_result.read().as_ref() {
                        None => rsx! {
                            EmptyScanState {}
                        },

                        Some(Err(error)) => rsx! {
                            ScanError {
                                message: error.to_string(),
                            }
                        },

                        Some(Ok(scan)) => rsx! {
                            ScanResult {
                                scan: scan.clone(),
                                selected_directory,
                            }
                        },
                    }
                }
            }

            div {
                class: "mt-6 border-t border-white/[0.08] pt-6",

                if let Some(error) = finish_error() {
                    p {
                        class: "mb-4 rounded-xl border border-red-400/20 bg-red-500/10 p-4 text-sm text-red-200",
                        "{error}"
                    }
                }

                Button {
                    label: if scanning() {
                        "Scanning..."
                    } else if finishing() {
                        "Finishing setup..."
                    } else if has_successful_scan {
                        "Continue"
                    } else {
                        "Scan repository"
                    },
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Lg,
                    full_width: true,
                    balanced: true,
                    class: Some("h-14 px-6 shadow-lg shadow-[color:var(--bg-accent)]/20".to_string()),
                    icon: Some(Icon {
                        kind: if has_successful_scan { IconKind::Arrow } else { IconKind::Search },
                        position: IconPosition::Right,
                        class: Some("size-4".to_string()),
                    }),
                    on_click: {

                        let project_id = project_id.clone();

                        move |_| {
                            let current_scan_result = scan_result.read().clone();
                            let scan = match current_scan_result {
                                Some(Ok(scan)) => Some(scan),
                                Some(Err(error)) => {
                                    finish_error.set(Some(error.to_string()));
                                    return;
                                }
                                None => None,
                            };

                            let Some(scan) = scan else {
                                start_scan(
                                    project_id.clone(),
                                    scanning,
                                    scan_result,
                                    selected_directory,
                                );

                                return;
                            };

                            let Some(locales_path) = selected_directory() else {
                                finish_error.set(Some("Choose a translation directory first.".to_string()));
                                return;
                            };

                            let source_locale = scan
                                .directories
                                .iter()
                                .find(|directory| directory.path == locales_path)
                                .and_then(|directory| directory.locales.first().cloned())
                                .or(scan.suggested_source_locale.clone())
                                .unwrap_or_else(|| "en-US".to_string());

                            let project_id = project_id.clone();
                            let navigator = navigator.clone();

                            finishing.set(true);
                            finish_error.set(None);

                            spawn(async move {
                                match finish_onboarding(project_id, locales_path, source_locale).await {
                                    Ok(_) => {
                                        finishing.set(false);
                                        navigator.replace("/app");
                                    }
                                    Err(error) => {
                                        finishing.set(false);
                                        finish_error.set(Some(error.to_string()));
                                    }
                                }
                            });
                        }
                    },
                }

                if has_successful_scan {
                    button {
                        class: "mt-4 w-full text-center text-sm font-medium text-[color:var(--text-muted)] transition hover:text-[color:var(--bg-accent)]",
                        type: "button",
                        onclick: {
                            let project_id = project_id.clone();

                            move |_| {
                                start_scan(
                                    project_id.clone(),
                                    scanning,
                                    scan_result,
                                    selected_directory,
                                );
                            }
                        },

                        "Scan again"
                    }
                }
            }
        }
    }
}

#[component]
fn ScanCardHeader() -> Element {
    rsx! {
        div {
            class: "flex items-start justify-between gap-6",

            div {
                h2 {
                    class: "font-serif text-3xl leading-none tracking-[-0.04em] text-[color:var(--text-primary)]",
                    "Detect translations"
                }

                p {
                    class: "mt-3 text-sm leading-6 text-[color:var(--text-secondary)]",
                    "Find .ftl files, then choose the directory that belongs to this project."
                }
            }

            div {
                class: "flex items-center gap-2 pt-1",

                span {
                    class: "flex size-7 items-center justify-center rounded-full border border-white/[0.12] bg-white/[0.04] text-xs font-bold text-[color:var(--text-muted)]",
                    "1"
                }

                span {
                    class: "h-px w-8 bg-[var(--bg-accent)]/40",
                }

                span {
                    class: "flex size-7 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white shadow-lg shadow-[color:var(--bg-accent)]/20",
                    "2"
                }
            }
        }
    }
}

#[component]
fn EmptyScanState() -> Element {
    rsx! {
        div {
            class: "rounded-2xl border border-white/[0.08] bg-white/[0.02] p-5",

            div {
                class: "flex items-center gap-4",

                div {
                    class: "flex size-11 shrink-0 items-center justify-center rounded-2xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/10 text-[color:var(--bg-accent)]",

                    RenderIcon {
                        kind: IconKind::FileText,
                        class: Some("size-5".to_string()),
                    }
                }

                div {
                    p {
                        class: "text-sm font-semibold text-[color:var(--text-primary)]",
                        "Ready to scan"
                    }

                    p {
                        class: "mt-1 text-sm leading-6 text-[color:var(--text-muted)]",
                        "BiteFluent will infer possible translation roots from detected .ftl files."
                    }
                }
            }
        }
    }
}

#[component]
fn ScanSkeleton() -> Element {
    rsx! {
        div {
            class: "space-y-3",

            for _ in 0..3 {
                Skeleton {
                    suspend: true,
                    class: "h-16 w-full rounded-xl border border-white/[0.08] bg-white/[0.02]".to_string(),
                    div {}
                }
            }
        }
    }
}

#[component]
fn ScanError(message: String) -> Element {
    rsx! {
        div {
            class: "rounded-xl border border-red-400/20 bg-red-500/10 p-4 text-sm leading-6 text-red-200",
            "{message}"
        }
    }
}

#[component]
fn ScanResult(scan: ProjectScanDto, selected_directory: Signal<Option<String>>) -> Element {
    let selected_path = selected_directory();

    let visible_files = selected_path
        .as_ref()
        .map(|path| files_for_directory(&scan.files, path))
        .unwrap_or_else(|| scan.files.clone());

    rsx! {
        div {
            class: "space-y-5",

            ScanSummary {
                files_count: scan.files.len(),
                selected_path: selected_path.clone(),
                selected_files_count: visible_files.len(),
            }

            if !scan.directories.is_empty() {
                div {
                    class: "space-y-2",

                    p {
                        class: "text-xs font-semibold uppercase tracking-[0.18em] text-[color:var(--text-muted)]",
                        "Translation directory"
                    }

                    for directory in scan.directories.iter().take(5) {
                        DirectoryRow {
                            directory: directory.clone(),
                            selected: selected_path
                                .as_ref()
                                .map(|path| path == &directory.path)
                                .unwrap_or(false),
                            on_select: move |path| {
                                selected_directory.set(Some(path));
                            },
                        }
                    }
                }
            }

            if scan.directories.len() > 5 {
                p {
                    class: "text-center text-sm text-[color:var(--text-muted)]",
                    "Showing the most likely directories."
                }
            }
        }
    }
}
#[component]
fn ScanSummary(
    files_count: usize,
    selected_path: Option<String>,
    selected_files_count: usize,
) -> Element {
    rsx! {
        div {
            class: "rounded-2xl border border-white/[0.08] bg-white/[0.02] p-5",

            div {
                class: "flex items-start gap-4",

                div {
                    class: "flex size-11 shrink-0 items-center justify-center rounded-2xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/10 text-[color:var(--bg-accent)]",

                    RenderIcon {
                        kind: IconKind::CheckNoCircle,
                        class: Some("size-5".to_string()),
                    }
                }

                div {
                    class: "min-w-0 flex-1",

                    p {
                        class: "text-sm font-semibold text-[color:var(--text-primary)]",
                        "Found {files_count} Fluent files"
                    }

                    p {
                        class: "mt-2 text-sm leading-6 text-[color:var(--text-muted)]",
                        if let Some(path) = selected_path.as_ref() {
                            "Using {selected_files_count} files from {path}."
                        } else {
                            "Choose the directory BiteFluent should use."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DirectoryRow(
    directory: FluentDirectoryDto,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let row_class = if selected {
        "border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/[0.06]"
    } else {
        "border-white/[0.08] bg-white/[0.02] hover:border-white/[0.14] hover:bg-white/[0.035]"
    };

    let check_class = if selected {
        "flex size-6 shrink-0 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white"
    } else {
        "flex size-6 shrink-0 items-center justify-center rounded-full border border-white/[0.16] bg-white/[0.02]"
    };

    rsx! {
        button {
            class: "group flex min-h-16 w-full items-center gap-4 rounded-xl border px-4 py-3 text-left transition {row_class}",
            type: "button",
            onclick: {
                let path = directory.path.clone();

                move |_| {
                    on_select.call(path.clone());
                }
            },

            span {
                class: "{check_class}",

                if selected {
                    RenderIcon {
                        kind: IconKind::CheckNoCircle,
                        class: Some("size-3".to_string()),
                    }
                }
            }

            RenderIcon {
                kind: IconKind::File,
                class: Some("size-5 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            span {
                class: "min-w-0 flex-1",

                span {
                    class: "block truncate text-sm font-semibold text-[color:var(--text-primary)]",
                    "{directory.path}"
                }

                span {
                    class: "mt-1 block text-xs text-[color:var(--text-muted)]",
                    "{directory.files_count} files · {directory.locales.len()} locales"
                }
            }

            if directory.recommended {
                span {
                    class: "rounded-full border border-[color:var(--bg-accent)]/30 bg-[var(--bg-accent)]/10 px-2.5 py-1 text-xs font-medium text-[color:var(--bg-accent)]",
                    "Recommended"
                }
            }
        }
    }
}

#[component]
fn FluentFileRow(file: FluentFileDto) -> Element {
    rsx! {
        div {
            class: "flex h-14 items-center gap-3 rounded-xl border border-white/[0.08] bg-white/[0.02] px-4",

            RenderIcon {
                kind: IconKind::FileText,
                class: Some("size-4 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            span {
                class: "min-w-0 flex-1 truncate text-sm font-medium text-[color:var(--text-primary)]",
                "{file.path}"
            }

            if let Some(locale) = file.locale.as_ref() {
                span {
                    class: "rounded-full bg-white/[0.06] px-2.5 py-1 text-xs font-medium text-[color:var(--text-muted)]",
                    "{locale}"
                }
            }
        }
    }
}

fn files_for_directory(files: &[FluentFileDto], directory: &str) -> Vec<FluentFileDto> {
    if directory == "." {
        return files.to_vec();
    }

    let prefix = format!("{directory}/");

    files
        .iter()
        .filter(|file| file.path.starts_with(&prefix))
        .cloned()
        .collect()
}
