use std::{collections::HashMap, env};

fn main() {
  let current_dir = env::current_dir().unwrap();
  let ui_dir = current_dir.join("ui");
  let libraries_dir = ui_dir.join("libraries");

  let mut library_paths = HashMap::new();

  if libraries_dir.exists() {
    let entries = libraries_dir.read_dir().unwrap();

    for entry in entries.filter_map(Result::ok) {
      let file_type = entry.file_type().unwrap();

      if file_type.is_dir() {
        let path = entry.path();
        let lib_path = path.join("lib.slint");

        library_paths.insert(
          entry.file_name().to_string_lossy().to_string(),
          if lib_path.exists() { lib_path } else { path },
        );
      }
    }
  }

  let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);

  slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
}
