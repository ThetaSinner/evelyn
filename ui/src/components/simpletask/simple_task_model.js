var SimpleTaskModel = Backbone.Model.extend({
    defaults: {
        title: '_task',
        description: "_description",
        dueDate: new Date().toISOString()
    }
});
