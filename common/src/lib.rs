pub mod error;
pub mod state;
pub mod password;
pub mod jwt;

#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");
#[cfg(test)]
mod tests {

    use rust_i18n::{set_locale, t};

    use super::*;
    #[test]
    fn it_works() {
        set_locale("zh");
        let str1 = t!("common.error.Unauthorized", msg = "test");
        assert_eq!(str1, "未授权 test");
    }
}
