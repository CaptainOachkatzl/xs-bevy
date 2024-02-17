use xs_bevy::TodoList;

#[test]
fn todo_list_test() {
    let mut list = TodoList::<i32>::default();
    assert_eq!(list.get_new().len(), 0);

    list.push(1);
    assert_eq!(list.get_new().len(), 1);
    assert_eq!(list.get_new().len(), 0);
}
