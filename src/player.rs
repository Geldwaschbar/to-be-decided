use macroquad::prelude::*;

#[derive(Debug)]
pub struct Mail {
    pub id: usize,
    pub author: String,
    pub message: String,
}

impl Mail {
    pub fn new(author: String, message: String) -> Mail {
        Mail {
            id: 0,
            author,
            message,
        }
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

#[derive(Debug)]
pub struct Player {
    mails: Vec<Mail>,
    money: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            mails: Vec::new(),
            money: 0.,
        }
    }

    pub fn income(&mut self, value: f64) {
        self.money += value;
    }

    pub fn can_buy(&self, value: f64) -> bool {
        self.money >= value
    }

    pub fn add_mail(&mut self, mut mail: Mail) {
        mail.id = self.mails.len();
        self.mails.push(mail);
    }

    pub fn fetch_mails(&self) -> &Vec<Mail> {
        &self.mails
    }
}
