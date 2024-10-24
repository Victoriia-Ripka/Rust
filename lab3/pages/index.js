import React, { useState, useEffect } from "react";
import { MdDelete, MdEdit } from "react-icons/md";
import axios from "axios";
import { format } from "date-fns"
import CheckBox from '../components/ChechBox'

const App = () => {
    const [todos, setTodos] = useState([]);
    const [todosCopy, setTodosCopy] = useState([]);
    const [todoInput, setTodoInput] = useState("");
    const [editIndex, setEditIndex] = useState(-1);
    const [search, setSearch] = useState("");
    const [count, setCount] = useState(0);

    useEffect(() => {
        const loadTodos = async () => {
            try {
                const response = await axios.get('http://127.0.0.1:8080/todos/load');
                setTodos(response.data);
                setTodosCopy(response.data); // Update the local copy as well
            } catch (error) {
                console.log("Error loading saved todos:", error);
            }
        };

        loadTodos();
    }, []);

    const fetchTodos = async () => {
        try {
            const response = await axios.get("http://127.0.0.1:8080/todos");
            const data = response.data;
            console.log(data);
            setTodos(data);
            setTodosCopy(data); 
        } catch (error) {
            console.log(error);
        }
    }

    useEffect(() => {
        fetchTodos();
    }, [count]);

    const addTodos = async () => {
        if (!todoInput) {
            alert("Todo title cannot be empty");
            return;
        }

        try {
            if (editIndex === -1) {
                // Add new todo
                const response = await axios.post("http://127.0.0.1:8080/todos", {
                    title: todoInput,
                    completed: false,
                });
                const newTodo = response.data;
                setTodos([...todos, newTodo]);
                setTodosCopy([...todosCopy, newTodo]);
                setTodoInput("");
            } else {
                // Update existing todo
                const todoToUpdate = { ...todos[editIndex], title: todoInput };
                const response = await axios.put(`http://127.0.0.1:8080/todos/${todoToUpdate.id}`, todoToUpdate);
                const updatedTodo = response.data;

                const updatedTodos = [...todos];
                updatedTodos[editIndex] = updatedTodo; 
                setTodos(updatedTodos);
                setTodosCopy(updatedTodos);
                setEditIndex(-1);
                setTodoInput("");
            }
            setCount(count + 1);
        } catch (error) {
            console.log("Error adding/updating todo:", error);
        }
    }

    const toggleCompleted = async (index) => {
        try {
            const todoToUpdate = { ...todos[index], completed: !todos[index].completed };
            const response = await axios.put(`http://127.0.0.1:8080/todos/${todoToUpdate.id}`, { completed: todoToUpdate.completed });
            const updatedTodos = [...response.data];
            setTodos(updatedTodos);
            setTodosCopy(updatedTodos);
        } catch (error) {
            console.log("Error toggling todo completion:", error);
        }
    }

    const deleteTodo = async (id) => {
        try {
            await axios.delete(`http://127.0.0.1:8080/todos/${id}`);
            const updatedTodos = todos.filter((todo) => todo.id !== id);
            setTodos(updatedTodos);
            setTodosCopy(updatedTodos);
            setCount(count - 1);
        } catch (error) {
            console.log("Error deleting todo:", error);
        }
    }

    const searchTodo = (e) => {
        const searchTerm = e.target.value.toLowerCase();
        setSearch(searchTerm);
        if (searchTerm === "") {
            setTodos(todosCopy); 
        } else {
            const filteredTodos = todosCopy.filter(todo => todo.title.toLowerCase().includes(searchTerm));
            setTodos(filteredTodos);
        }
    }

    const formatDate = (dateString) => {
        try {
            if (!dateString) return "Invalid Date";
            const cleanDateString = dateString.split('.')[0] + 'Z';
            const date = new Date(cleanDateString);
            return isNaN(date.getTime()) ? "Invalid Date" : format(date, "yyyy-MM-dd HH:mm:ss");
        } catch (error) {
            console.log(error);
            return "Invalid Date";
        }
    }

    const editTodo = (index) => {
        setTodoInput(todos[index].title);
        setEditIndex(index);
    }

    const renderTodos = (todosToRender) => {
        return todosToRender.map((todo, index) => (
            <li key={todo.id} className="li">
                <CheckBox toggleCompleted={() => toggleCompleted(index)} todo={todo} />
                <span className="todo-text">
                    {`${todo.title} ${formatDate(todo.created_at)}`}
                </span>
                <span className="span-button" onClick={() => deleteTodo(todo.id)}>
                    <MdDelete />
                </span>
                <span className="span-button" onClick={() => editTodo(index)}>
                    <MdEdit />
                </span>
            </li>
        ));
    };

    const saveTodos = async () => {
        try {
            const response = await axios.post('http://127.0.0.1:8080/todos/save');
            console.log(response.data);
        } catch (error) {
            console.log("Error saving todos:", error);
        }
    };

    const loadTodosFromFile = async () => {
        try {
            const response = await axios.get('http://127.0.0.1:8080/todos/load');
            setTodos(response.data);
            setTodosCopy(response.data);
        } catch (error) {
            console.log("Error loading todos:", error);
        }
    };

    return (
        <section className="main-section">
            <div className="todo-app">
                <div className="input-section">
                    <input
                        type="text"
                        id="todoInput"
                        placeholder="Add a todo"
                        value={todoInput || ""} // Ensure it's always controlled
                        onChange={(e) => setTodoInput(e.target.value)}
                    />
                    <button onClick={addTodos} className="add">
                        {editIndex === -1 ? "Add" : "Update"}
                    </button>

                    <input
                        type="text"
                        id="searchInput"
                        placeholder="Search"
                        value={search}
                        onChange={searchTodo}
                    />
                </div>

                <div className="todos">
                    <ul className="todo-list">
                        {renderTodos(todos)}
                    </ul>

                    {todos.length === 0 && (
                        <div className="no-todos">
                            <h1 className="not-found">No todos found</h1>
                        </div>
                    )}
                </div>
            </div>

            <button onClick={saveTodos}>Save Todos</button>
            <button onClick={loadTodosFromFile}>Load Todos from File</button>
        </section>
    );
}

export default App;
