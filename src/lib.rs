use ctor::{ctor, dtor};
use std::env;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

static IS_JAVA: AtomicBool = AtomicBool::new(false);

#[ctor]
fn resctrl() {
    let arg0 = env::args_os().next().unwrap();
    if arg0.to_str().unwrap().ends_with("java") {
        IS_JAVA.store(true, Ordering::SeqCst);
    } else {
        return;
    }

    let cos = env::var("RESCTRL_COS").expect("Need to set RESCTRL_COS");
    eprintln!("Operating on COS {}", cos);
    let schemata_path = format!("/sys/fs/resctrl/{}/schemata", cos);
    if let Ok(mut s) = env::var("RESCTRL_SCHEMATA") {
        eprintln!("Setting schemata to {}", s);
        s.push_str("\n");
        fs::write(schemata_path, s).unwrap();
    }
    let tasks_path = format!("/sys/fs/resctrl/{}/tasks", cos);
    let pid = std::process::id();
    eprintln!("Setting tasks to {}", pid);
    fs::write(&tasks_path, format!("{}\n", pid)).unwrap();
    let schemata = fs::read_to_string(format!("/sys/fs/resctrl/{}/schemata", cos)).unwrap();
    let tasks = fs::read_to_string(&tasks_path).unwrap();
    eprintln!(
        "Hello {}, schemata {}, tasks {}",
        std::process::id(),
        schemata,
        tasks
    );
}

#[dtor]
fn resctrl_teardown() {
    if !IS_JAVA.load(Ordering::SeqCst) {
        return;
    }
    let cos = env::var("RESCTRL_COS").unwrap_or("dacapo_cos".into());
    eprintln!("Operating on COS {}", cos);
    let tasks_path = format!("/sys/fs/resctrl/{}/tasks", cos);
    eprintln!("Clearing tasks");
    fs::write(&tasks_path, "").unwrap();
}
