use anyways::{ext::AuditExt, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::{Path, PathBuf};
use log::info;
use tokio::fs::{create_dir_all, read_dir, OpenOptions};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Clone)]
pub struct AssetManager {
    assets: PathBuf,
    data: PathBuf,
    config: PathBuf,
    cache: PathBuf,
}

impl AssetManager {
    pub async fn new() -> Result<AssetManager> {
        #[cfg(debug_assertions)]
            let comp = AssetManager {
            assets: PathBuf::from("./assets"),
            data: PathBuf::from("./home/data"),
            config: PathBuf::from("./home/config"),
            cache: PathBuf::from("./home/cache"),
        };
        #[cfg(not(debug_assertions))]
            let comp = AssetManager {
            assets: PathBuf::from("./assets"),
            data: dirs::data_dir()
                .expect("Could not find the data directory")
                .join("pitaya"),
            config: dirs::config_dir()
                .expect("Could not find the data directory")
                .join("pitaya"),
            cache: dirs::cache_dir()
                .expect("Could not find the cache directory")
                .join("pitaya"),
        };

        create_dir_all(&comp.assets).await?;
        create_dir_all(&comp.data).await?;
        create_dir_all(&comp.config).await?;
        create_dir_all(&comp.cache).await?;

        info!("Created asset manager");

        Ok(comp)
    }

    pub fn get_dir(&self, loc: Location) -> &Path {
        match loc {
            Location::Assets => &self.assets,
            Location::Data => &self.data,
            Location::Config => &self.config,
            Location::Cache => &self.cache,
        }
    }

    pub async fn read_dir<P: AsRef<Path>>(
        &self,
        loc: Location,
        path: P,
    ) -> io::Result<Vec<PathBuf>> {
        let source = self.get_dir(loc);
        let path = source.join(path);

        let mut paths = Vec::new();
        let mut read_dir = read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            let path = path.strip_prefix(&source).expect("Failed to strip prefix");
            paths.push(path.to_path_buf());
        }

        Ok(paths)
    }

    pub async fn get_data<P, S>(&self, loc: Location, path: P) -> Result<S>
        where
            P: AsRef<Path>,
            S: Serialize + DeserializeOwned + Default,
    {
        if self.contains_file(loc, &path).await {
            let data = self
                .read_file(loc, path)
                .await
                .wrap_err("Failed to read file")?;
            let value: S = serde_json::from_slice(&data).wrap_err("Failed to deserialize data")?;
            Ok(value)
        } else {
            let default = S::default();
            self.save_data(loc, path, &default)
                .await
                .wrap_err("Failed to create new data")?;
            Ok(default)
        }
    }

    pub async fn save_data<P, S>(&self, loc: Location, path: P, value: &S) -> Result<()>
        where
            P: AsRef<Path>,
            S: Serialize + DeserializeOwned + Default,
    {
        let data = serde_json::to_vec_pretty(value).wrap_err("Failed to serialize data")?;
        self.write_file(loc, path, &data)
            .await
            .wrap_err("Failed to write to file")?;
        Ok(())
    }

    pub async fn contains_file<P: AsRef<Path>>(&self, loc: Location, path: P) -> bool {
        let source = self.get_dir(loc);
        let path = source.join(&path);
        tokio::fs::metadata(&path).await.is_ok()
    }

    pub async fn read_file<P: AsRef<Path>>(&self, loc: Location, path: P) -> io::Result<Vec<u8>> {
        let source = self.get_dir(loc);
        let path = source.join(&path);
        let mut file = tokio::fs::File::open(path).await?;
        let length = file.metadata().await.map(|meta| meta.len()).unwrap_or(0);
        let mut buf = Vec::with_capacity(length as usize);
        file.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    pub async fn write_file<P: AsRef<Path>>(
        &self,
        loc: Location,
        path: P,
        data: &[u8],
    ) -> io::Result<()> {
        let source = self.get_dir(loc);
        let path = source.join(&path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(path)
            .await?;
        file.write_all(data).await?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Location {
    /// The Asset location which contains this installations assets.
    Assets,
    /// The User location holds **potentially sensitive**  information.
    /// Stuff like login information is contained here.
    /// If you want to expose simple configuration options, Use [Location::Config] instead.
    Data,
    /// The Config location holds configuration files that should **not contain sensitive information**.
    /// Things like current theme set and accessibility features are saved here.
    /// If you are storing sensitive information, use [Location::User] instead.
    Config,
    /// The Cache location holds files to speed-up lookup of files or reduce api requests.
    /// This folder may be cleared by the user at any time but hopefully not often.
    Cache,
}
