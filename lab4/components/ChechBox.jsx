import React from "react";

const CheckBox = ({ toggleCompleted, index, todo }) => {
    return (
        <label className="container">
            <input
                checked={todo.completed} 
                type="checkbox"
                onChange={() => toggleCompleted(index)} 
            />
        </label>
    );
};

export default CheckBox;
