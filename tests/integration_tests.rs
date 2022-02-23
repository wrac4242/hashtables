use hashtables;

#[test]
fn inserting_getting() {
    let mut ht = hashtables::Hashtable::new(1);
    ht.insert("key", "value").unwrap();
    assert_eq!(ht.get("key"), Ok("value"));
}

#[test]
fn inserting_getting_multiple() {
    let mut ht = hashtables::Hashtable::new(4);
    ht.insert("key1", "value1").unwrap();
    ht.insert("key2", "value2").unwrap();
    ht.insert("key3", "value3").unwrap();
    ht.insert("key4", "value4").unwrap();

    assert_eq!(ht.get("key1"), Ok("value1"));
    assert_eq!(ht.get("key2"), Ok("value2"));
    assert_eq!(ht.get("key3"), Ok("value3"));
    assert_eq!(ht.get("key4"), Ok("value4"));
}

#[test]
fn errors_insert_if_key_blank() {
    let mut ht = hashtables::Hashtable::new(1);
    assert_eq!(ht.insert("", "value4"), Err(()));
}

#[test]
fn errors_insert_not_set_key() {
    let mut ht = hashtables::Hashtable::new(1);
    assert_eq!(ht.insert("key", ""), Err(()));
}

#[test]
fn inserting_removing() {
    let mut ht = hashtables::Hashtable::new(1);
    ht.insert("key", "value").unwrap();
    ht.remove("key").unwrap();
    assert_eq!(ht.get("key"), Err(()));
}

#[test]
fn inserting_getting_collision() {
    let mut ht = hashtables::Hashtable::new(3);
    ht.insert("key1", "value1").unwrap();
    ht.insert("key2", "value2").unwrap();
    ht.insert("key3", "value3").unwrap();
    ht.insert("key4", "value4").unwrap();

    assert_eq!(ht.get("key1"), Ok("value1"));
    assert_eq!(ht.get("key2"), Ok("value2"));
    assert_eq!(ht.get("key3"), Ok("value3"));
    assert_eq!(ht.get("key4"), Ok("value4"));
}

#[test]
fn inserting_removing_collision() {
    let mut ht = hashtables::Hashtable::new(3);
    ht.insert("key1", "value1").unwrap();
    ht.insert("key2", "value2").unwrap();
    ht.insert("key3", "value3").unwrap();
    ht.insert("key4", "value4").unwrap();

    ht.remove("key4").unwrap();

    assert_eq!(ht.get("key1"), Ok("value1"));
    assert_eq!(ht.get("key2"), Ok("value2"));
    assert_eq!(ht.get("key3"), Ok("value3"));
    assert_eq!(ht.get("key4"), Err(()));
}

#[test]
fn errors_remove_not_set_key() {
    let mut ht = hashtables::Hashtable::new(1);
    assert_eq!(ht.remove(""), Err(()));
}

#[test]
fn errors_remove_wrong_key() {
    let mut ht = hashtables::Hashtable::new(1);
    assert_eq!(ht.remove("key"), Err(()));
}

#[test]
fn two_identical_keys() {
    let mut ht = hashtables::Hashtable::new(1);
    ht.insert("key", "value").unwrap();
    ht.insert("key", "value1").unwrap();
    assert_eq!(ht.get("key"), Ok("value1"));
}
