evelynDesktopApp.component('createTodoListComponent', {
    template: '@@include(cleanHtml("src/components/todo-list/create-todo-list.partial.html"))',

    controller: function($scope, $state, alertify, serverBridgeService) {
        $scope.title = "";

        $scope.create = function() {
            serverBridgeService.send_to_server('/todolist/create', {
                Title: $scope.title,
            }, function (response) {
                if (response.Error === null) {
                    alertify.success("Sucessfully created todo list");
                    $state.go('dashboard.viewtodolist', {
                        todoListId: response.TodoListId,
                    });
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                    console.log(response.Error);
                }
            });
        };
    }
});
