use crate::communication::packet::Reader;
use crate::event::controller::user_info::UserInfoEvent;


pub fn parse(mut reader: Reader) -> UserInfoEvent {
    UserInfoEvent{
        user_id: reader.read_uint32().unwrap(),
        username: reader.read_string().unwrap(),
        figure: reader.read_string().unwrap(),
        gender: reader.read_string().unwrap(),
        motto: reader.read_string().unwrap(),
        real_name: reader.read_string().unwrap(),
        direct_mail: reader.read_bool().unwrap(),
        respects_received: reader.read_uint32().unwrap(),
        respects_remaining: reader.read_uint32().unwrap(),
        respects_pet_remaining: reader.read_uint32().unwrap(),
        stream_publishing_allowed: reader.read_bool().unwrap(),
        last_access_date: reader.read_string().unwrap(),
        can_change_name: reader.read_bool().unwrap(),
        safety_locked: reader.read_bool().unwrap()
    }
}