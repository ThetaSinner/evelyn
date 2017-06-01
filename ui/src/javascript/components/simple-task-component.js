evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function ($scope) {
        this.$onChanges = function (changes) {
            var simpleTaskView = new SimpleTaskView({
                collection: new SimpleTaskCollection(changes.simpleTasks.currentValue)
            });
        };
    }
});
