#[cfg(test)]

mod tests {
    use crate::lib;

    #[test]
    fn test_add_task() {
        let mut todo = lib::TodoList::new();
        todo.add_task(1,"Hello", "World");
        assert_eq!(todo.tasks[0].Description, "Hello");
        assert_eq!(todo.tasks[0].Title, "World");
        assert_eq!(1, todo.tasks.len());

        todo.add_task(2,"second", "this is second task");
        assert_eq!(todo.tasks[1].Description, "second");
        assert_eq!(todo.tasks[1].Title, "this is second task");
        assert_eq!(2, todo.tasks.len());
    }

    #[test]
    fn test_remove_task() {
        let mut todo = lib::TodoList::new();
        todo.add_task(1,"Hello", "World");
        assert_eq!(todo.tasks.len(), 1);
        todo.remove_task(1);
        assert_eq!(todo.tasks.len(), 0);
    }
}
