use std::collections::HashMap;
use crate::communication::packet::Reader;
use crate::event::controller::room::{RoomUser, RoomUsersEvent};

pub fn parse(mut reader: Reader) -> RoomUsersEvent {
    let total_users = reader.read_uint32().unwrap();
    let mut room_users: HashMap<u32, RoomUser> = HashMap::new();
    for _ in 0..total_users {
        let user = RoomUser{
            user_id: reader.read_uint32().unwrap(),
            username: reader.read_string().unwrap(),
            custom: reader.read_string().unwrap(),
            figure: reader.read_string().unwrap(),
            room_unit_id: reader.read_uint32().unwrap(),
            x: reader.read_uint32().unwrap(),
            y: reader.read_uint32().unwrap(),
            z: reader.read_string().unwrap(),
            direction: reader.read_uint32().unwrap(),
            user_type: reader.read_uint32().unwrap()
        };

        // doe ik even niks mee, want is niet echt nuttig atm.
        if user.user_type == 1 {
            let _ = reader.read_string().unwrap(); // sex
            let _ = reader.read_uint32().unwrap(); // group_id
            let _ = reader.read_uint32().unwrap(); // group_status
            let _ = reader.read_string().unwrap(); // group_name
            let _ = reader.read_string().unwrap(); // swim_figure
            let _ = reader.read_uint32().unwrap(); // activity_points
            let _ = reader.read_bool().unwrap(); // is_moderator

            room_users.insert(user.user_id.clone(), user);
        } else if user.user_type == 2 {
            // we're not doing anything with pets right now, but we'll parse it anyway as it's
            // required lmao
            let _ = reader.read_uint32().unwrap(); // sub_type
            let _ = reader.read_uint32().unwrap(); // owner_id
            let _ = reader.read_string().unwrap(); // owner_name
            let _ = reader.read_uint32().unwrap(); // rarity_level
            let _ = reader.read_bool().unwrap(); // has_saddle
            let _ = reader.read_bool().unwrap(); // is_riding
            let _ = reader.read_bool().unwrap(); // can_breed
            let _ = reader.read_bool().unwrap(); // can_harvest
            let _ = reader.read_bool().unwrap(); // can_revive
            let _ = reader.read_bool().unwrap(); // has_breeding_permission
            let _ = reader.read_uint32().unwrap(); // pet_level
            let _ = reader.read_string().unwrap(); // pet_posture
        } else if user.user_type == 4 {
            // rentable bot
            let _ = reader.read_string().unwrap(); // sex
            let _ = reader.read_uint32().unwrap(); // owner_id
            let _ = reader.read_string().unwrap(); // owner_name

            let total_skills = reader.read_uint32().unwrap();
            let mut j = 0;
            while j < total_skills {
                let _ = reader.read_uint16(); // skill
                j += 1;
            }
        }
    }

    RoomUsersEvent{
        total_users,
        users: room_users.drain().map(|(_, room_user)| room_user).collect()
    }
}