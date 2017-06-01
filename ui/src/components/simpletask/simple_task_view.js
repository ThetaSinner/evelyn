var SimpleTaskView = Backbone.View.extend({
    el: '#simple-tasks-container',

    template: Handlebars.compile('@@include(cleanHtml("src/components/simpletask/simple_task_view.template.html"))'),

    initialize: function() {
        this.render();
    },

    render: function() {
        this.$el.html(this.template({SimpleTasks: this.collection.toJSON()}));
        return this;
    }
});
