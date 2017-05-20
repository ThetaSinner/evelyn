var SimpleTaskView = Backbone.View.extend({
    el: '#simple_task_view',

    template: _.template('@@include(clean("simpletask/simple_task_create.partial.html"))', {variable: "data"}),

    initialize: function() {
        this.render();
    },

    render: function() {
        this.$el.html(this.template({
            title : this.model.get('title'),
            description : this.model.get('description'),
            dueDate : this.model.get('dueDate'),
            completed: this.model.get('completed')
        }));
    },
});
