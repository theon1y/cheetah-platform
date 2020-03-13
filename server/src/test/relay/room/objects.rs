
use crate::relay::room::groups::AccessGroups;
use crate::relay::room::object::GameObject;
use crate::relay::room::objects::Objects;

#[test]
fn should_insert_objects() {
    let mut objects = setup_game_objects();
    let owner_id = 10;
    let object = GameObject::new(owner_id, 10, AccessGroups::new());
    let object_id = object.id;
    objects.insert(object);
    assert_eq!(objects.get(object_id).is_some(), true)
}


#[test]
fn should_remove_objects_by_owner() {
    let mut objects = setup_game_objects();
    let owner_a_id = 1;
    let owner_b_id = 2;
    objects.insert(GameObject::new(owner_a_id, 10, AccessGroups::new()));
    objects.insert(GameObject::new(owner_b_id, 10, AccessGroups::new()));
    assert_eq!(objects.len(), 2);
    objects.remove_objects_by_owner(owner_a_id);
    assert_eq!(objects.len(), 1);
}

fn setup_game_objects() -> Objects {
    let objects = Objects::new();
    return objects;
}