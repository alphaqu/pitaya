use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::{read_dir};
use anyways::ext::AuditExt;
use tokio::io::AsyncReadExt;

// use tokio_stream::wrappers::ReadDirStream;
// use zip::result::ZipResult;
// use zip::ZipArchive;
#[derive(Clone)]
pub struct AssetComp {
    assets: PathBuf,
    user: PathBuf,
    cache: PathBuf,
}

impl AssetComp {
    pub fn new() -> AssetComp {
        // if in dev
        #[cfg(debug_assertions)]
        return AssetComp {
            assets: PathBuf::from("./assets"),
            user: PathBuf::from("../../../home/user"),
            cache: PathBuf::from("../../../home/cache"),
        };
        #[cfg(not(debug_assertions))]
        panic!()
    }

    pub fn get_dir(&self, loc: Location) -> &Path {
        match loc {
            Location::Assets => &self.assets,
            Location::User => &self.user,
            Location::Cache => &self.cache,
        }
    }

    pub async fn read_dir<P: AsRef<Path>>(&self, loc: Location, path: P) -> io::Result<Vec<PathBuf>> {
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

    pub async fn read_file<P: AsRef<Path>>(&self, loc: Location, path: P) -> io::Result<Vec<u8>> {
        let source = self.get_dir(loc);
        let path = source.join(&path);
        let mut file = tokio::fs::File::open(path).await?;
        let length = file.metadata().await.map(|meta| meta.len()).unwrap_or(0);
        let mut buf = Vec::with_capacity(length as usize);
        file.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}

#[derive(Copy, Clone)]
pub enum Location {
    /// Folder for Pitaya installation assets. Stuff like icons and such are here.
    Assets,
    /// Pitaya user folder. Things like preferences lie here.
    User,
    /// Holds things like map tile cache.
    Cache,
}

// pub struct Archive {
//     path: PathBuf,
//     //Zip(RwLock<ArchiveZip>),
//     //Direct(ArchiveDirect),
// }
//
// impl Archive {
//     pub async fn list_dir(&self, path: PathBuf) -> io::Result<Vec<PathBuf>> {
//         let path = self.path.join(path);
//
//         let mut paths = Vec::new();
//         let mut read_dir = read_dir(path).await?;
//         while let Some(entry) = read_dir.next_entry().await? {
//             paths.push(entry.path());
//         }
//
//         Ok(paths)
//     }
//     pub async fn get_asset(&self, path: PathBuf) -> io::Result<Vec<u8>> {
//         let path = self.path.join(path);
//         let mut file = tokio::fs::File::open(path).await?;
//         let length = file.metadata().await.map(|meta| meta.len()).unwrap_or(0);
//         let mut buf = Vec::with_capacity(length as usize);
//         file.read_to_end(&mut buf).await?;
//         Ok(buf)
//     }
//
//    //pub async fn list_dir(&self, path: PathBuf) -> ZipResult<Vec<PathBuf>> {
//    //    match self {
//    //        Archive::Zip(_) => {}
//    //        Archive::Direct(_) => {}
//    //    }
//    //}
//
//    //pub async fn get_asset(&self, path: PathBuf) -> ZipResult<Vec<u8>> {
//    //    let value = match self {
//    //        Archive::Zip(archive) => archive.write().await.get_asset(path).await?,
//    //        Archive::Direct(archive) => archive.get_asset(path).await?,
//    //    };
//
//    //    Ok(value)
//    //}
// }
// pub struct ArchiveDirect {
//     path: PathBuf,
// }
//
// impl ArchiveDirect {
//     pub async fn list_dir(&self, path: PathBuf) -> ZipResult<Vec<PathBuf>> {
// 	    let path = self.path.join(path);
//
// 	    let mut paths = Vec::new();
// 	    let mut read_dir = read_dir(path).await?;
// 	    while let Some(entry) = read_dir.next_entry().await? {
// 		    paths.push(entry.path());
// 	    }
//
// 	    Ok(paths)
//     }
//     pub async fn get_asset(&self, path: PathBuf) -> io::Result<Vec<u8>> {
//         let path = self.path.join(path);
//         let mut file = tokio::fs::File::open(path).await?;
//         let length = file.metadata().await.map(|meta| meta.len()).unwrap_or(0);
//         let mut buf = Vec::with_capacity(length as usize);
//         file.read_to_end(&mut buf).await?;
//         Ok(buf)
//     }
// }

// pub struct ArchiveZip {
//     archive: PathBuf,
//     zip: ZipArchive<File>,
// }
//
// impl ArchiveZip {
//     pub async fn get_asset(&mut self, path: PathBuf) -> ZipResult<Vec<u8>> {
//         let name = path
//             .to_str()
//             .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, ""))?;
//         let mut file = self.zip.by_name(name)?;
//         let mut buf = Vec::with_capacity(file.size() as usize);
//         file.read_to_end(&mut buf)?;
//         Ok(buf)
//     }
// }