import React from 'react';
import ReactDOM from 'react-dom';

class SimpleTask extends React.Component {
  constructor(props) {
    super(props);
    this.switchToEdit = this.switchToEdit.bind(this);
    this.save = this.save.bind(this);
    this.handleTitleChange = this.handleTitleChange.bind(this);
    this.handleDescriptionChange = this.handleDescriptionChange.bind(this);
    this.state = {
      task: {
        title: this.props.title,
        description: this.props.description,
      },
      isEdit: false,
    };
  }

  render() {
    if (this.state.isEdit) {
      return (
        <form>
          <input type="text" onChange={this.handleTitleChange} value={this.state.task.title} />
          <textarea onChange={this.handleDescriptionChange} value={this.state.task.description}></textarea>
          <button className="button" onClick={this.save}>Save</button>
        </form>
      );
    }
    else {
      return (
        <div>
          <h4>{this.state.task.title}</h4>
          <p>{this.state.task.description}</p>
          <button onClick={this.switchToEdit}>
            <i className="fi-wrench"></i>
          </button>
        </div>
      );
    }
  }

  switchToEdit(e) {
    this.setState((prevState) => ({
      task: prevState.task,
      isEdit: true,
    }));
  }

  save(e) {
    this.setState((prevState) => ({
      task: {
        title: this.state.task.title,
        description: this.state.task.description,
      },
      isEdit: false,
    }));
  }

  handleTitleChange(e) {
    var newTitle = e.target.value;
    this.setState((prevState) => ({
      task: {
        title: newTitle,
        description: prevState.task.description,
      },
      isEdit: prevState.isEdit,
    }));
  }

  handleDescriptionChange(e) {
    var newDescription = e.target.value;
    this.setState((prevState) => ({
      task: {
        title: prevState.task.title,
        description: newDescription,
      },
      isEdit: prevState.isEdit,
    }));
  }
}

ReactDOM.render(
  <SimpleTask title="the title" description="the description" />,
  document.getElementById('root')
);
