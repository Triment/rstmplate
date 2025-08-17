use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signature, VerifyingKey};
use std::{path::{Path}};

const EMBEDDED_PUBKEY_BASE64: &str = "VBmqo7KvrtbgFncLBj70Jgl+vEBWsiJB/GvCKcVuSfg=";

pub fn load_plugins() -> Result<Vec<Box<dyn plugin::Plugin>>, Box<dyn std::error::Error>> {
    let mut plugins: Vec<Box<dyn plugin::Plugin>> = Vec::new();
    for entry in std::fs::read_dir("plugins").unwrap() {
        let entry = entry.unwrap();
        #[cfg(target_os = "linux")]
        let ext_name = "so";
        #[cfg(target_os = "windows")]
        let ext_name = "dll";
        #[cfg(target_os = "macos")]
        let ext_name = "dylib";

        if entry
            .path()
            .extension()
            .map_or(false, |ext| ext == ext_name)
        {
            let plugin_path = entry.path();
            let sig_path = plugin_path.with_extension("sig");
            if verify_plugin_signature(&plugin_path, &sig_path, EMBEDDED_PUBKEY_BASE64).is_ok()
            {
                if let Ok(plugin) = unsafe { libloading::Library::new(&plugin_path) } {
                    if let Ok(create_fn) =
                        unsafe { plugin.get::<plugin::PluginCreate>(b"create_plugin") }
                    {
                        plugins.push(create_fn());
                    }
                }
            }
        }
    }
    Ok(plugins)
}

pub fn verify_plugin_signature<P: AsRef<Path>>(
    file: P,
    sig_file: P,
    pubkey_b64: &str,
) -> Result<()> {
    let data = std::fs::read(&file)?;
    let sig_bytes = std::fs::read(&sig_file)?;
    let pk_bytes = STANDARD.decode(pubkey_b64.trim())?;

    if pk_bytes.len() != 32 {
        anyhow::bail!("pubkey must be 32 bytes");
    }
    if sig_bytes.len() != 64 {
        anyhow::bail!("signature must be 64 bytes");
    }

    let pk = VerifyingKey::from_bytes(&pk_bytes.try_into().unwrap())?;
    let sig = Signature::from_bytes(&sig_bytes.try_into().unwrap());
    pk.verify_strict(&data, &sig)
        .context("signature verify failed")?;
    Ok(())
}
