use std::thread;

use rocket::http::Status;
use rocket::log;
use rocket::tokio;
use rocket::serde::json::to_string;

use bambangshop_receiver::{Result, compose_error_response, APP_CONFIG, REQWEST_CLIENT};
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;

pub struct NotificationService;

impl NotificationService {
    
}