use hashtables;

#[test]
fn inserting_removing() {
	let mut ht = hashtables::Hashtable::new();
	ht.insert("key", "value").unwrap();
    assert_eq!(ht.get("key"), Ok("value"));
}

#[test]
fn inserting_removing_multiple() {
	let mut ht = hashtables::Hashtable::new();
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
#[should_panic]
fn errors_if_key_blank() {
	let mut ht = hashtables::Hashtable::new();
	ht.insert("", "value4").unwrap();
}

#[test]
#[should_panic]
fn errors_not_set_key() {
	let mut ht = hashtables::Hashtable::new();
	ht.insert("key", "").unwrap();
}
