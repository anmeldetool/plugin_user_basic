use utils::{error::AuthError, App, AuthPlugin, Plugin, UserPlugin};

pub struct Users {}

impl Users {
    pub fn new() -> Self {
        Users {}
    }
}

impl Plugin for Users {
    fn init(&self, app: &App) -> Result<(), ()> {
        let res: Result<(), ()> = match app.register_user_plugin(self) {
            Ok(_) => Ok(()),
            Err(_) => return Err(()),
        };

        let res = match app.register_auth_plugin(self) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };

        res
    }
    fn shutdown(&self, _app: &App) -> Result<(), ()> {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn Plugin> {
        Box::new(Users {})
    }
}

impl UserPlugin for Users {
    fn render_input(&self) -> String {
        todo!()
    }

    fn render_edit(&self, _user_id: &str) -> String {
        todo!()
    }

    fn render_display(&self, _user_id: &str) -> String {
        todo!()
    }
}

impl AuthPlugin for Users {
    fn authenticate(&self, _name: &str, _password: &str) -> Result<(), AuthError> {
        todo!()
    }
}
