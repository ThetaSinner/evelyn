var SimpleTaskView = Backbone.View.extend({
    template: _.template('@@include(cleanHtml("src/components/simpletask/simple_task_view.partial.html"))', {
        variable: "data"
    }),

    render: function() {
        this.$el.html(this.template(this.model.toJSON()));
        return this;
    }
});

var simpleTaskView = new SimpleTaskCollectionView({
    collection: simpleTaskCollection
});
