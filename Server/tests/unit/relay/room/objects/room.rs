use cheetah_relay_common::room::access::AccessGroups;
use cheetah_relay_common::room::fields::GameObjectFields;

use crate::unit::relay::room::setup_and_two_client;

#[test]
fn create_client_game_object() {
    let (mut room, client, _) = setup_and_two_client();
    let id = room.create_client_game_object(
        &client.clone(),
        255,
        AccessGroups::from(0b100000),
        GameObjectFields::default(),
    ).ok().unwrap();
    assert_eq!(format!("{:x}", id), "1000000ff");
    assert_eq!(room.objects.get(id).is_some(), true);
}


#[test]
fn delete_client_game_object() {
    let (mut room, client, _) = setup_and_two_client();
    let id = room.create_client_game_object(
        &client.clone(),
        255,
        AccessGroups::from(0b100000),
        GameObjectFields::default(),
    ).ok().unwrap();
    let game_object = room.objects.get_by_owner(&client.clone(), 255).unwrap();
    room.delete_game_object(&(*(*(game_object.clone())).borrow()));
    assert_eq!(room.objects.get(id).is_none(), true);
}


#[test]
fn create_client_game_object_when_missing_client_group() {
    let (mut room, client, _) = setup_and_two_client();
    let result = room
        .create_client_game_object(
            &client.clone(),
            255,
            AccessGroups::from(0b1000),
            GameObjectFields::default(),
        );
    assert_eq!(result.is_err(), true)
}


#[test]
fn create_root_game_object() {
    let (mut room, _, _) = setup_and_two_client();
    let id = room.create_root_game_object(
        255,
        AccessGroups::default(),
        GameObjectFields::default(),
    ).ok().unwrap();
    assert_eq!(id, 255);
}