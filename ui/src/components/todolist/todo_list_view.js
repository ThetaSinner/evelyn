var TodoListView = Backbone.View.extend({
    el: '#active_todo_list',

    template: _.template($("#todo_list_template_definition").text(), {variable: "data"}),

    initialize: function() {
        this.render();
    },

    render: function() {
        this.$el.html(this.template({
            title: this.model.get('title'),
            todo_items: this.model.get('todo_items'),
        }));
    },
});
