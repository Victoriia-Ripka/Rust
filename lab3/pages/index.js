import React, { useState, useEffect } from "react";
import { MdDelete, MdEdit, MdConfirmationNumber } from "react-icons/md";
import axios from "axios";
import {format} from "date-fns"

const index = () => {
    const [editText, setEditText] = useState("");
    const [todos, setTodos] = useState([]);
    const [todosCopy, setTodosCopy] = useState(todos);
    const [todoInput, setTodoInput] = useState("");
    const [editIndex, setEditIndex] = useState(-1);
    const [searchInput, setSearchInput] = useState("");
    const [searchResult, setSearchResult] = useState([]);

    const [count, setCount] = useState(0);
    const [search, setSearch] = useState("");
    const [searchItem, setSearchItem] = useState(search);

    const editTodo = (index) => {
        setTodoInput(todos[index].text);
        setEditIndex(index);
    }

    const fetchTodos = async () => {
        try {
            const response = await axios.get("http://127.0.0.1:8080/todos");
            const data = response.data;
            console.log(data)
            setTodos(data);
            setTodosCopy(data);
        } catch (error) {
            console.log(error)
        }
    }

    const addTodos = async () => {
        try {
            if (editIndex === -1) {
                // add new todo
                const response = await axios.post("http://127.0.0.1:8080/todos", {
                    title: todoInput,
                    completed: false,
                });
                const data = response.data;
                setTodos(data);
                setTodosCopy(data);
                setTodoInput("");
                setCount(count + 1)
            } else {
                // update existing todo
                const todoToUpdate = {...todos[editIndex], title: todoInput}
                const response = await axios.put(`http://127.0.0.1:8080/todos/${todoToUpdate.id}`, {
                    todoToUpdate
                });
                console.log(response)
                const updatedTodos = [...todos]
                updatedTodos[editIndex] = response.config.data
                setTodos(updatedTodos)
                // setCount(count + 1)
                setEditIndex(-1);
                setTodoInput("");
            }
        } catch (error) {
            console.log(error)
        }
    }

    const toggleCompleted = async (index) => {
        try {
            const todoToUpdate = { ...todos[index], completed: !todos[index].completed }
            
            const response = await axios.put(`http://127.0.0.1:8080/todos/${todoToUpdate.id}`)
            const data = response.data

            const updatedTodos = [...todos]
            updatedTodos[index] = data
  
            setTodos(updatedTodos)
            setCount(count + 1)
        } catch (error) {
            console.log(error)
        }
    }    

    const deleteTodo = async (id) => {
        try {
            const response = await axios.delete(`http://127.0.0.1:8080/todos/${id}`);
            setTodos(todos.filter((todo) => todo.id !== id));
            setCount(count - 1);
            setTodosCopy(todos.filter((todo) => todo.id !== id));
        } catch (error) {
            console.log(error)
        }
    }    

    const searchTodo = (e) => {
        const results = todos.filter((todo) => {
            todo.title.toLowerCase().includes(e.target.value.toLowerCase())
        })

        setSearchInput(results);
    }

    const formedatDate = (dateString) => {
        try {
            const cleanDateString = dateString.split('.')[0] + 'Z';
            const date = new Date(cleanDateString);
            return isNaN(date.getTime()) ? "Invalid Date" : format(date, "yyyy-MM-dd HH:mm:ss");
        } catch (error) {
            console.log(error)
        }
    }

    useEffect(() => {
        fetchTodos();
    }, [count]);

    const renderTodos = (todosToRender) => {
        return todosToRender.map((todo, index) => (
            <li key={index} classname="li">
                <label htmlFor="" classname="form-check-label"></label>

                <span classname="todo-text">
                    {`${todo.title} ${formedatDate(todo.created_at)}`}
                </span>

                <span classname="span-button" onClick={() => deleteTodo(todo.id)}>
                    <i className="fa-solid fa-trash">
                        <MdDelete />
                    </i>
                </span>

                <span classname="span-button" onClick={() => setEditIndex(index)}>
                    <i className="fa-solid fa-trash">
                        <MdEdit />
                    </i>
                </span>
            </li>
        ));
    };

    const onHandleSearch = (value) => {
        const filteredTodo = todos.filter((todo) => {
            ({ title }) => title.toLowerCase.includes(value.toLowerCase())
        })

        if(filteredTodo.length === 0) {
            setTodos(todosCopy)
        } else {
            setTodos(filteredTodo)
        }
            
    }

    const onClearSearch = () => {
        if (todos.length && todosCopy.length) {
            setTodos(todosCopy)
        }
    }

    useEffect(() => {
        const timer = setTimeout(() => {
            setSearchItem(search)
        }, 1000)

        return clearTimeout(timer)
    }, [searchItem]);

    useEffect(() => {
        if (search) {
            onHandleSearch(search)
        } else {
            onClearSearch()
        }
    }, [search])

    
    return <section classsname="main-section">
        <div classsname="todo-app">
            
            <div  classsname="input-section">
                <input type="text" id="todoInput" placeholder="Add a todo" value={todoInput} onChange={(e) => setTodoInput(e.target.value)} />
                <button onClick={() => addTodos()} classsname="add">{editIndex === -1 ? "Add" : "Update"}</button>

                <input type="text" id="searchInput" placeholder="Search" value={search} onChange={(e) => setSearch(e)} />
                <button onClick={() => {}} >Search</button>
            </div>

            <div classname="todos">
                <ul classname="todo-list">
                    {renderTodos(todos)}
                </ul>

                {todos.length === 0 && (
                    <div classname="no-todos">
                        <h1 classname="not-found">No todos found</h1>
                    </div>
                )}
            </div>

        </div>
    </section>;
}

export default index;