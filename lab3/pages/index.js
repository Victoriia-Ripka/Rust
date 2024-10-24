import React, { useState, useEffect } from "react";
import { MdDelete, MdEdit, MdConfirmationNumber } from "react-icons/md";
import axios from "axios";
import {format} from "date-fns"

const index = () => {
    const [editText, setEditText] = useState();
    const [todos, setTodos] = useState([]);
    const [todosCopy, setTodosCopy] = useState(todos);
    const [todoInput, setTodoInput] = useState("");
    const [editIndex, setEditIndex] = useState(-1);
    const [searchInput, setSearchInput] = useState("");
    const [searchResult, setSearchResult] = useState([]);

    const [count, setCount] = useState(0);
    const [search, setSearch] = useState("");
    const [searchItem, setSearchItem] = useState(search);
    

    useEffect(() => {
        
    }, [count]);

    const editTodo = (index) => {
        setTodoInput(todos[index].text);
        setEditIndex(index);
    }

    const fetchTodos = async () => {
        try {
            const response = await axios.get("http://127.0.0.1:8080/todos");
            const data = response.data;
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
                    text: todoInput,
                    completed: false,
                });
                const data = response.data;
                setTodos(data);
                setTodosCopy(data);
                setTodoInput("");
            } else {
                // update existing todo
                const todoToUpdate = {...todos[editIndex], title: todoInput}
                const response = await axios.put(`http://127.0.0.1:8080/todos/${todoToUpdate.id}`, {
                    todoToUpdate
                });
                console.log(response)
                const updatedTodos = [...todos]
                updatedTodos[editIndex] = response.data
                setTodos(updatedTodos)
                setCount(count + 1)
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
            const date = new Date(dateString);
            return isNan(date.getTime()) ? "Invalid Date" : format(date, "yyyy-MM-dd HH:mm:ss");;
        } catch (error) {
            console.log(error)
        }
    }
    
    return <div>index</div>;
}

export default index;