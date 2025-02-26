// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{Model, ModelRc, VecModel};

slint::include_modules!();

fn main() -> anyhow::Result<()> {
  let app = MainWindow::new()?;

  app.on_get_display_number({
    let handle = app.as_weak();
    move || {
      let app = handle.unwrap();
      let display = app.invoke_get_display_text();

      display.replace(',', ".").parse().unwrap()
    }
  });

  app.on_function_clicked({
    let handle = app.as_weak();
    move |func: Function| {
      let app = handle.unwrap();
      match func {
        Function::Reset => app.invoke_reset(),
        Function::Delete => {
          let number = app.get_number().to_string();
          let number = &number[..number.len() - 1];
          let number = number.trim_end_matches(',');

          app.invoke_set_number(if number.is_empty() {
            0.0
          } else {
            number.parse().unwrap()
          });
        }
        Function::Clear if app.get_calculated() || app.get_number() == 0.0 => app.invoke_reset(),
        Function::Clear => app.invoke_set_number(0.0),
      }
    }
  });

  app.on_add_current_to_history({
    let handle = app.as_weak();
    move || {
      let app = handle.unwrap();
      let history: VecModel<Process> = app.get_history().iter().collect();
      history.push(Process {
        p_number: app.get_p_number(),
        number: app.get_number(),
        operation: match app.get_operation() {
          Operation::None => unreachable!(),
          op => op,
        },
        result: app.get_result(),
      });

      app.set_history(ModelRc::new(history));
    }
  });

  app.run()?;

  Ok(())
}
