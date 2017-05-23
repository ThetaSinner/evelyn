var simpleTaskCollection = new SimpleTaskCollection([
    {title: 'Task 1', description: "Lorem"},
    {title: 'Task 2', description: "Ipsum"},
    {title: 'Task 3', description: "Dolor"},
    {title: 'Task 4', description: ""},
]);

var simpleTaskView = new SimpleTaskCollectionView({collection: simpleTaskCollection});