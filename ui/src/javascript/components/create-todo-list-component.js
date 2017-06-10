evelynDesktopApp.component('createTodoListComponent', {
    template: '@@include(cleanHtml("src/components/todo-list/create-todo-list.partial.html"))',

    controller: function($scope, $state, serverBridgeService) {
        $scope.title = "";

        $scope.create = function() {
            serverBridgeService.send_to_server('/todolist/create', {
                Title: $scope.title,
            }, function (response) {
                if (response.Error === null) {
                    $state.go('dashboard.viewtodolist', {
                        todoListId: response.TodoListId,
                    });
                }
                else {
                    console.log(response.Error);
                }
            });
        };
    }
});
