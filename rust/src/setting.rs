use godot::classes::ConfigFile;
use godot::prelude::*;
use snafu::{OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error when saving config"))]
    Save,
    #[snafu(display("Error when loading config"))]
    Load,
    #[snafu(display("Error when loading config: converting failed"))]
    Convert { source: ConvertError },
    #[snafu(display("Error when getting singleton"))]
    Singleton,
}

#[derive(GodotClass)]
#[class(base=Object)]
pub struct Setting {
    inner: Gd<ConfigFile>,
    base: Base<Object>,
}

fn load_config() -> Result<Gd<ConfigFile>, Error> {
    let mut inner = ConfigFile::new_gd();
    let error = inner.load("user://settings.cfg");
    match error {
        godot::global::Error::OK => Ok(inner),
        _ => Err(Error::Load),
    }
}

fn default_config() -> Gd<ConfigFile> {
    let mut inner = ConfigFile::new_gd();
    inner.set_value("global", "sensitivity", &Variant::from(0.01));
    inner
}

#[godot_api]
impl Setting {
    pub fn save(&mut self) -> Result<(), Error> {
        let error = self.inner.save("user://settings.cfg");
        match error {
            godot::global::Error::OK => Ok(()),
            _ => Err(Error::Save),
        }
    }

    pub fn set_value(&mut self, section: &str, key: &str, value: impl ToGodot) {
        self.inner.set_value(section, key, &Variant::from(value));
    }

    pub fn set_and_save<T: ToGodot>(
        &mut self,
        section: &str,
        key: &str,
        value: T,
    ) -> Result<(), Error> {
        self.set_value(section, key, value);
        self.save()
    }

    pub fn get_value<T: FromGodot>(&self, section: &str, key: &str) -> Result<T, Error> {
        let variant = self.inner.get_value(section, key);
        let value = variant.try_to_relaxed().context(ConvertSnafu)?;
        Ok(value)
    }

    pub fn singleton() -> Result<Gd<Self>, Error> {
        let object = godot::classes::Engine::singleton()
            .get_singleton("Setting")
            .context(SingletonSnafu)?;
        Ok(object.cast())
    }
}

#[godot_api(secondary)]
impl Setting {
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.set_and_save("global", "sensitivity", sensitivity).expect("Expect ok")
    }

    pub fn get_sensitivity(&self) -> f32 {
        self.get_value("global", "sensitivity").expect("Expect ok")
    }
}

#[godot_api]
impl IObject for Setting {
    fn init(base: Base<Self::Base>) -> Self {
        match load_config() {
            Ok(inner) => Self { inner, base },
            Err(_) => Self {
                inner: default_config(),
                base,
            },
        }
    }
}
