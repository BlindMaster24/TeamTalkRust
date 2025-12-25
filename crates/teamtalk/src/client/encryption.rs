use super::Client;
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    pub fn set_encryption_context(&self, cert_file: &str, key_file: &str) -> bool {
        let mut ctx = unsafe { std::mem::zeroed::<ffi::EncryptionContext>() };
        let c = cert_file.tt();
        let k = key_file.tt();

        unsafe {
            ctx.szCertificateFile[..c.len().min(512)].copy_from_slice(&c[..c.len().min(512)]);
            ctx.szPrivateKeyFile[..k.len().min(512)].copy_from_slice(&k[..k.len().min(512)]);
            ffi::api().TT_SetEncryptionContext(self.ptr, &ctx) == 1
        }
    }
}
