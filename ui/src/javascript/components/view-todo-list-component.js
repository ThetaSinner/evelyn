evelynDesktopApp.component('viewTodoListComponent', {
    template: '@@include(cleanHtml("src/components/todo-list/view-todo-list.partial.html"))',

    bindings: {
        todoList: '<',
    },

    controller: function($scope, $state, alertify, serverBridgeService) {
        $scope.addItem = function() {
            var todoListId = $scope.$ctrl.todoList.TodoListId;

            // TODO check new item isn't empty.
            serverBridgeService.send_to_server('/todolist/item/add', {
                TodoListId: todoListId,
                TodoListItem: {
                    Text: $scope.newItem,
                    IsDone: false,
                },
            }, function(response) {
                if (response.Error === null) {
                    alertify.success("Successfully added item to todolist");
                    $state.reload('dashboard.viewtodolist');
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                    console.log(response.Error);
                }
            });
        };

        $scope.updateDone = function(index) {
            var todoListId = $scope.$ctrl.todoList.TodoListId;
            var isDone = $scope.$ctrl.todoList.TodoListItems[index].IsDone;

            serverBridgeService.send_to_server('/todolist/item/update', {
                TodoListId: todoListId,
                ItemIndex: index,
                IsDone: isDone,
            }, function (response) {
                if (response.Error === null) {
                    alertify.success("Todo list Item updated");
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                    console.log(response.Error);
                }
            });
        }
    }
});
