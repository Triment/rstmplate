pub fn load_plugins() -> Result<Vec<Box<dyn plugin::Plugin>>, Box<dyn std::error::Error>> {
    let mut plugins: Vec<Box<dyn plugin::Plugin>> = Vec::new();
    for entry in std::fs::read_dir("plugins").unwrap() {
        let entry = entry.unwrap();
        #[cfg(target_os = "linux")]
        let ext_name = "so";
        #[cfg(target_os= "windows")]
        let ext_name = "dll";
        #[cfg(target_os = "macos")]
        let ext_name = "dylib";

        if entry.path().extension().map_or(false, |ext| ext == ext_name) {
            let plugin_path = entry.path();
            let plugin_name = plugin_path.file_stem().unwrap().to_str().unwrap().to_string();
            // 加载插件
            if let Ok(plugin) = unsafe { libloading::Library::new(plugin_path) } {
                if let Ok(create_fn) = unsafe { plugin.get::<plugin::PluginCreate>(b"create_plugin") } {
                    plugins.push(create_fn());
                }
            }
        }
    }
    Ok(plugins)
}