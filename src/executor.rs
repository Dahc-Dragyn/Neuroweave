use std::path::PathBuf;
use tokio::fs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileTransaction {
    pub path: PathBuf,
    pub original_content: String,
}

impl FileTransaction {
    pub async fn begin(path: &str) -> Result<Self, std::io::Error> {
        let path_buf = PathBuf::from(path);
        let original_content = match fs::read_to_string(&path_buf).await {
            Ok(content) => content,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e),
        };
        Ok(Self {
            path: path_buf,
            original_content,
        })
    }

    pub async fn write(&self, new_content: &str) -> Result<(), std::io::Error> {
        fs::write(&self.path, new_content).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn rollback(&self) -> Result<(), std::io::Error> {
        fs::write(&self.path, &self.original_content).await?;
        Ok(())
    }
}
