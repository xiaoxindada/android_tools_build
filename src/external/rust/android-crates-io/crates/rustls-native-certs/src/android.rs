use crate::{CertPaths, CertificateResult};

pub fn load_native_certs() -> CertificateResult {
    CertPaths {
        file: None,
        dirs: vec!["/apex/com.android.conscrypt/cacerts".into()],
    }
    .load()
}
