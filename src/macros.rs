macro_rules! gen_hash_map_helpers {
    ($([$key:ident, $output:ident]),*) => {
        paste::paste! {
            impl PBXHashMap {
                $(
                    #[doc = "Get " $key " if value is of type " $output]
                    pub fn [<get_ $key>](&self, key: &str) -> Option<&$output> {
                        self.0.get(key)?.[<as_ $key>]()
                    }

                    #[doc = "Try get " $key " of type " $output]
                    pub fn [<try_get_ $key>](&self, key: &str) -> Result<&$output> {
                        let value = self.try_get_value(key)?;
                        value.[<as_ $key>]().ok_or_else(|| anyhow::anyhow!("expected value to be {}, got {:?}", stringify!($key), value))
                    }

                    #[doc = "remove " $key " of type " $output]
                    pub fn [<remove_ $key>](&mut self, key: &str) -> Option<$output> {
                        self.remove_value(key).map(|v| v.[<try_into_ $key>]().ok()).flatten()
                    }

                    #[doc = "Try remove " $key " of type " $output]
                    pub fn [<try_remove_ $key>](&mut self, key: &str) -> Result<$output> {
                        let value = self.try_remove_value(key)?;
                        value.[<try_into_ $key>]()
                    }

                )*
            }
        }
    };
}
pub(crate) use gen_hash_map_helpers;
