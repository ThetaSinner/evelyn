var SimpleTaskView = Backbone.View.extend({
    el: '#simple_task_view',

    template: _.template($("#simple_task_template").text(), {variable: "data"}),

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
