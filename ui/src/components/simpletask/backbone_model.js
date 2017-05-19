var SimpleTaskModel = Backbone.Model.extend({
    defaults : {
        title : "_title",
        description : "_description",
        dueDate : "_due_date",
        completed: false
    }
});
