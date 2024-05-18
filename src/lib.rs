use std::path::PathBuf;

pub fn log_perf(log_file: &PathBuf) {
    log_file.exists().then(|| backup_file(log_file));
}

fn backup_file(log_file: &PathBuf) {

    let mut backup_file = log_file.clone();
    let get_new_backup_file = |i: i64| {
        backup_file.set_extension(format!("{}-{}", log_file.extension().unwrap().to_string_lossy(), i));
        (!backup_file.exists()).then(|| backup_file.clone())
    };
    (1..)
        .find_map(get_new_backup_file)
        .and_then(|backup_file| {
            println!(
                "Backing up {} to {}",
                log_file.display(),
                backup_file.display()
            );
            std::fs::rename(log_file, backup_file).ok()
        })
        .unwrap_or_else(|| {
            panic!(
                "Could not backup {} to {}",
                log_file.display(),
                backup_file.display()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_backup(count: u32) {
        let test_dir = PathBuf::from("test_dir");
        let _ = std::fs::remove_dir_all(&test_dir);
        std::fs::create_dir(&test_dir).unwrap();
        let test_file = test_dir.join("test.txt");
        for i in 1..=count {
            std::fs::File::create(&test_file).unwrap();
            backup_file(&test_file);
            let expected_file = test_file.with_extension(format!("txt-{}", i));
            assert!(expected_file.exists());
        }
        assert!(!test_file.exists());
        assert!(!test_file.with_extension(format!("txt-{}", count+1)).exists());
        std::fs::remove_dir_all(&test_dir).unwrap();
    }
    #[test]
    fn backup_one_file() {
        test_backup(1);
    }
    #[test]
    fn backup_two_file() {
        test_backup(2);
    }
    #[test]
    fn backup_three_file() {
        test_backup(3);
    }
}
