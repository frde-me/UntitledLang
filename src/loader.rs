use std::collections::HashMap;
use std::fs::File;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::hash::Hash;

pub trait HasKey {
    type Key = String;
    fn create_key(&self) -> Self::Key;
}

pub trait Load {
    type Object;
    type Key = String;
    fn load(key: impl Into<Self::Key>) -> &'static Self::Object;
}

pub fn load_from_folder<K: Eq + Hash, T: serde::de::DeserializeOwned + HasKey<Key=K>>(path: &str) -> std::io::Result<HashMap<K, T>> {
    let files = std::fs::read_dir(path)?;

    Ok(files.filter_map(|file| {
        match file {
            Ok(file) => Some(file),
            Err(e) => None
        }
    }).filter_map(|file| match File::open(file.path()){
        Ok(file) => Some(file),
        Err(e) => None
    }).map(|file| {
        let t: T = serde_json::from_reader(file).unwrap();
        t
    })
        .map(|d| (d.create_key(), d))
        .collect::<HashMap<K, T>>())
}

#[macro_export]
macro_rules! identifier {
    ($struct_name: ty, $field_name: tt, $name: tt) => {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl<S: Into<String>> From<S> for $name {
            fn from(s: S) -> Self {
                $name(s.into())
            }
        }
        impl HasKey for $struct_name {
            type Key = $name;
            fn create_key(&self) -> Self::Key {
                self.$field_name.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! loader {
    ($struct_name: ty, $key_name: ty, $path: literal) => {
        impl Load for $struct_name {
            type Object = Self;
            type Key = $key_name;

            fn load(key: impl Into<Self::Key>) -> &'static Self::Object {
                &values[&key.into()]
            }
        }

        lazy_static! {
            static ref values: HashMap<$key_name, $struct_name> = crate::loader::load_from_folder::<$key_name, $struct_name>($path).unwrap();
        }
    };
}
