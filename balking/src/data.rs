use std::{
    fs::File,
    io::Write,
    sync::{Mutex, MutexGuard},
    thread,
};

pub struct Data {
    file_name: String,
    body: Mutex<Body>,
}

struct Body {
    content: String,
    changed: bool,
}

impl Data {
    pub fn new(file_name: &str, content: &str) -> Self {
        Self {
            file_name: file_name.into(),
            body: Mutex::new(Body {
                content: content.into(),
                changed: true,
            }),
        }
    }

    pub fn change(&self, new_content: &str) {
        let mut body = self.body.lock().unwrap();

        body.content = new_content.into();
        body.changed = true;
    }

    pub fn save(&self) {
        let mut body = self.body.lock().unwrap();

        if !body.changed {
            return;
        }

        self.do_save(&body);
        body.changed = false;
    }

    fn do_save(&self, body: &MutexGuard<Body>) {
        println!(
            "{} calls do_save, content = {}",
            thread::current().name().unwrap(),
            body.content
        );

        File::create(&self.file_name)
            .unwrap()
            .write_all(body.content.as_bytes())
            .unwrap();
    }
}
