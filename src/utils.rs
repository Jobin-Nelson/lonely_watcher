use crate::prelude::*;
use std::path::Path;

pub fn backup_file(log_file: &Path) -> Result<()> {
    let mut backup_file = log_file.to_path_buf();
    let get_new_backup_file = |i: i64| {
        backup_file.set_extension(format!(
            "{}-{}",
            log_file.extension().unwrap().to_string_lossy(),
            i
        ));
        (!backup_file.exists()).then(|| backup_file.clone())
    };
    let new_backup_file = (1..).find_map(get_new_backup_file);
    match new_backup_file {
        Some(b) => {
            println!("Backup up {log_file:?} to {b:?}");
            std::fs::rename(log_file, b).map_err(Error::Io)
        },
        None => Err(Error::BackupFailed {
            source_file: log_file.to_path_buf(),
            destination_file: new_backup_file,
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;
    fn test_backup(test_dir: &str, count: u32) {
        let test_dir = PathBuf::from(test_dir);
        let _ = std::fs::remove_dir_all(&test_dir);
        std::fs::create_dir(&test_dir).unwrap();
        let test_file = test_dir.join("test.txt");
        for i in 1..=count {
            std::fs::File::create(&test_file).unwrap();
            backup_file(&test_file).unwrap();
            let expected_file = test_file.with_extension(format!("txt-{}", i));
            assert!(expected_file.exists());
        }
        assert!(!test_file.exists());
        assert!(!test_file
            .with_extension(format!("txt-{}", count + 1))
            .exists());
        std::fs::remove_dir_all(&test_dir).unwrap();
    }
    #[test]
    fn backup_one_file() {
        test_backup("test_dir_1", 1);
    }
    #[test]
    fn backup_two_file() {
        test_backup("test_dir_2", 2);
    }
    #[test]
    fn backup_three_file() {
        test_backup("test_dir_3", 3);
    }
}
