use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::tg::TGMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Photo {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i32>,
    pub vcard: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub is_animated: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f32,
    pub y_shift: f32,
    pub scale: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGPhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGInvoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGSuccessfulPayment {
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<TGOrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGOrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<TGShippingAddress>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

pub fn unify_attachments(message: Option<TGMessage>) -> String {
    if message.is_none() {
        return String::new();
    }
    let message = message.unwrap();
    serde_json::to_string(&TGAttachment {
        audio: message.audio,
        document: message.document,
        photo: message.photo,
        sticker: message.sticker,
        video: message.video,
        video_note: message.video_note,
        voice: message.voice,
        caption: message.caption,
        contact: message.contact,
        location: message.location,
        venue: message.venue,
    })
    .unwrap()
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TGAttachment {
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
}
