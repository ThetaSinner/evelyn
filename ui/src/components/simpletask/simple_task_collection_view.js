var SimpleTaskCollectionView = Backbone.View.extend({
    el : "#simple_task_view",

    initialize : function () {
        this.listenTo( this.collection, 'add', this.render);
        this.render();
    },

    render: function () {
        this.$el.empty();
        this.collection.each(function (person) {
            var simpleTaskView = new SimpleTaskView({model: person});
            this.$el.append(simpleTaskView.render().el); // calling render method manually..
        }, this);
        return this; // returning this for chaining..
    }
});
