#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[tauri::command]
fn export_pdf(window: tauri::WebviewWindow, path: String) -> Result<(), String> {
    use webkit2gtk::PrintOperationExt;
    window
        .with_webview(move |webview| {
            let wv = webview.inner();
            let settings = gtk::PrintSettings::new();
            settings.set_printer("Print to File");
            settings.set("output-uri", Some(format!("file://{}", path).as_str()));
            settings.set("output-file-format", Some("pdf"));
            let op = webkit2gtk::PrintOperation::new(&wv);
            op.set_print_settings(&settings);
            op.print();
        })
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init());

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    let builder = builder.invoke_handler(tauri::generate_handler![export_pdf]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
