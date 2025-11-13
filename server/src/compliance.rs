use once_cell::sync::OnceCell;
use openai_sdk_rs::OpenAI;
use std::env;

#[derive(Debug)]
pub struct ComplianceSettings {
    api_key: Option<String>,
}

static SETTINGS: OnceCell<ComplianceSettings> = OnceCell::new();

pub fn init() -> &'static ComplianceSettings {
    SETTINGS.get_or_init(|| {
        let key = env::var("COMPLIANCE_API_KEY").ok();
        ComplianceSettings { api_key: key }
    })
}

impl ComplianceSettings {
    pub fn is_enabled(&self) -> bool {
        self.api_key.is_some()
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn client(&self) -> Option<OpenAI> {
        let key = self.api_key.as_deref()?;
        OpenAI::new(key).ok()
    }
}
