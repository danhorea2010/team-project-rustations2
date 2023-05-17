import React, { useEffect, useState } from 'react'
import './App.css'
import { useAppDispatch, useAppSelector } from './main'
import { loadTodos, Todo } from './store/todos'

function App() {
  const [count, setCount] = useState(0)

  const dispatch = useAppDispatch(); 
  const todosState = useAppSelector((state) => state.todos);

  useEffect(() => {
    dispatch(loadTodos(false));
  }, [])


  return (
    <div className="App">

      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      {todosState.todosLoading === false ? (
        todosState.todosList.map((item: Todo, index: any) => 
          <React.Fragment key={index} >
            <div className='card'>
              <div>Title: {item.title}</div>
              <div>Description: {item.description}</div>
            </div>
          </React.Fragment>
        )
      ) : null
      }
    </div>
  )
}

export default App
