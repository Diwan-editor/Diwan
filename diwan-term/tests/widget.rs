


#[cfg(test)]
mod widget {
    use std::sync::{Arc, Mutex};

    use anyhow::Error;
    use diwan::screen::{MainScreen, Modes};


    #[test]
    fn test_create_widget() -> Result<(), Error> {
        let dnbuffer = MainScreen::new_buffered_term()?;
        let content = Arc::new(Mutex::new(String::new()));
        let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
        let dnwidget = MainScreen::new_with_widget(dnbuffer, content.clone())?;

        //assert_eq!(*dnwidget.1.text.lock().unwrap(), *content.lock().unwrap());
        assert_eq!(dnwidget.1.mode, Modes::Normal);
        assert_eq!(dnwidget.1.cursor_x, 0);
        assert_eq!(dnwidget.1.cursor_y, 0);
        assert_eq!(*dnwidget.1.yank.lock().unwrap(), *yank.lock().unwrap());
        assert_eq!(dnwidget.1.status_bar.filename,  "[SCRATCH]".to_string());
        assert_eq!(dnwidget.1.status_bar.status_mode, Modes::Normal.to_string());
        Ok(())
    }

}
