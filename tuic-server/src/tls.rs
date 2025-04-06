use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use eyre::Context;
use notify::{RecursiveMode, Watcher as _};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    server::{ClientHello, ResolvesServerCert},
    sign::CertifiedKey,
};
use tracing::warn;

use crate::utils::{self, FutResultExt};

#[derive(Debug)]
pub struct CertResolver {
    cert_path: PathBuf,
    key_path: PathBuf,
    cert_key: RwLock<Arc<CertifiedKey>>,
}
impl CertResolver {
    pub async fn new(cert_path: &Path, key_path: &Path) -> eyre::Result<Arc<Self>> {
        let cert_key = load_cert_key(cert_path, key_path).await?;
        let resolver = Arc::new(Self {
            cert_path: cert_path.to_owned(),
            key_path: key_path.to_owned(),
            cert_key: RwLock::new(cert_key),
        });
        let resolver_clone = resolver.clone();
        tokio::spawn(async move {
            resolver_clone.start_watch().log_err().await;
        });
        Ok(resolver)
    }

    async fn start_watch(&self) -> eyre::Result<()> {
        let (mut watcher, mut rx) = utils::async_watcher().await?;

        watcher.watch(self.cert_path.as_ref(), RecursiveMode::NonRecursive)?;
        while (rx.recv().await).is_ok() {
            warn!("TLS cert-key reload");
            let cert_key = load_cert_key(&self.cert_path, &self.key_path).await?;
            if let Ok(mut guard) = self.cert_key.write() {
                *guard = cert_key;
            }
        }
        Ok(())
    }
}
impl ResolvesServerCert for CertResolver {
    fn resolve(&self, _: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        Some(self.cert_key.read().ok()?.deref().clone())
    }
}

async fn load_cert_key(cert_path: &Path, key_path: &Path) -> eyre::Result<Arc<CertifiedKey>> {
    let cert_chain = load_cert_chain(cert_path).await?;
    let der = load_priv_key(key_path).await?;

    #[cfg(feature = "aws-lc-rs")]
    let key = rustls::crypto::aws_lc_rs::sign::any_supported_type(&der)?;
    #[cfg(feature = "ring")]
    let key = rustls::crypto::ring::sign::any_supported_type(&der)?;

    let cert_key = CertifiedKey::new(cert_chain, key);
    Ok(Arc::new(cert_key))
}

async fn load_cert_chain(cert_path: &Path) -> eyre::Result<Vec<CertificateDer<'static>>> {
    let cert_chain = tokio::fs::read(cert_path)
        .await
        .context("failed to read certificate chain")?;
    let cert_chain = if cert_path.extension().is_some_and(|x| x == "der") {
        vec![CertificateDer::from(cert_chain)]
    } else {
        rustls_pemfile::certs(&mut &*cert_chain)
            .collect::<Result<_, _>>()
            .context("invalid PEM-encoded certificate")?
    };
    Ok(cert_chain)
}

async fn load_priv_key(key_path: &Path) -> eyre::Result<PrivateKeyDer<'static>> {
    let key = tokio::fs::read(key_path)
        .await
        .context("failed to read private key")?;
    let key = if key_path.extension().is_some_and(|x| x == "der") {
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key))
    } else {
        rustls_pemfile::private_key(&mut &*key)
            .context("malformed PKCS #1 private key")?
            .ok_or_else(|| eyre::Error::msg("no private keys found"))?
    };
    Ok(key)
}
