use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer, SignatureError};
use rand::rngs::OsRng;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name="plugin_signer", version, about="Ed25519 keygen & signer for plugins")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// 生成密钥对，输出到当前目录：ed25519_sk.bin (32B), ed25519_pk.bin (32B)
    Gen,
    /// 用私钥对文件签名，生成 <file>.sig (64B)
    Sign { file: PathBuf, #[arg(short, long)] key: Option<PathBuf> },
    /// 用公钥校验签名（自检）
    Verify { file: PathBuf, #[arg(short, long)] pubkey: Option<PathBuf>, #[arg(short, long)] sig: Option<PathBuf> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Gen => r#gen(),
        Cmd::Sign { file, key } => sign(file, key),
        Cmd::Verify { file, pubkey, sig } => verify(file, pubkey, sig),
    }
}

fn r#gen() -> Result<()> {
    let mut rng = OsRng;
    let sk = SigningKey::generate(&mut rng);
    let pk: VerifyingKey = sk.verifying_key();

    fs::write("ed25519_sk.bin", sk.to_bytes())?;
    fs::write("ed25519_pk.bin", pk.to_bytes())?;

    println!("Generated keys:");
    println!("  ed25519_sk.bin (keep secret)  base64: {}", STANDARD.encode(sk.to_bytes()));
    println!("  ed25519_pk.bin (embed in app) base64: {}", STANDARD.encode(pk.to_bytes()));
    Ok(())
}

fn sign(file: PathBuf, key: Option<PathBuf>) -> Result<()> {
    let sk_path = key.unwrap_or_else(|| "ed25519_sk.bin".into());
    let sk_bytes = fs::read(&sk_path).with_context(|| format!("read sk {}", sk_path.display()))?;
    if sk_bytes.len() != 32 { bail!("sk must be 32 bytes (found {})", sk_bytes.len()); }

    let sk = SigningKey::from_bytes(&sk_bytes.try_into().unwrap());
    let data = fs::read(&file).with_context(|| format!("read {}", file.display()))?;
    let sig: Signature = sk.sign(&data);

    let sig_path = file.with_extension(format!("{}sig", file.extension().map(|e| format!("{}. ", e.to_string_lossy())).unwrap_or_default()).replace(". ", "."));
    // 更稳妥：直接 <file>.sig
    let sig_path = file.with_extension("").with_extension("sig");
    fs::write(&sig_path, sig.to_bytes())?;
    println!("Wrote signature: {}", sig_path.display());
    println!("Signature (base64): {}", STANDARD.encode(sig.to_bytes()));
    Ok(())
}

fn verify(file: PathBuf, pubkey: Option<PathBuf>, sig: Option<PathBuf>) -> Result<()> {
    let pk_path = pubkey.unwrap_or_else(|| "ed25519_pk.bin".into());
    let sig_path = sig.unwrap_or_else(|| file.with_extension("").with_extension("sig"));

    let pk_bytes = fs::read(&pk_path)?;
    if pk_bytes.len() != 32 { bail!("pk must be 32 bytes (found {})", pk_bytes.len()); }
    let pk = VerifyingKey::from_bytes(&pk_bytes.try_into().unwrap())?;

    let data = fs::read(&file)?;
    let sig_bytes = fs::read(&sig_path)?;
    if sig_bytes.len() != 64 { bail!("sig must be 64 bytes (found {})", sig_bytes.len()); }
    let sig = Signature::from_bytes(&sig_bytes.try_into().unwrap());

    match pk.verify_strict(&data, &sig) {
        Ok(_) => { println!("OK: signature valid"); Ok(()) }
        Err(e) => Err(anyhow::anyhow!("Invalid signature: {:?}", e)),
    }
}
