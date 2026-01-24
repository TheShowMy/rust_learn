import { useState, useEffect } from 'react'
import './App.css'

interface Todo {
  id: number;
  user_id: number;
  title: string;
  completed: boolean;
}

function App() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'))
  const [todos, setTodos] = useState<Todo[]>([])
  const [newTodoTitle, setNewTodoTitle] = useState('')

  useEffect(() => {
    if (token) {
      fetchTodos()
    }
  }, [token])

  const fetchTodos = async () => {
    try {
      const response = await fetch('/todos', {
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      })
      if (response.ok) {
        const data = await response.json()
        setTodos(data)
      } else if (response.status === 401) {
        handleLogout()
      }
    } catch (error) {
      console.error('Error fetching todos:', error)
    }
  }

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      const response = await fetch('/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      })
      if (response.ok) {
        const data = await response.json()
        setToken(data.token)
        localStorage.setItem('token', data.token)
      } else {
        alert('登录失败')
      }
    } catch (error) {
      console.error('Error during login:', error)
      alert('登录出错')
    }
  }

  const handleAddTodo = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!newTodoTitle.trim()) return

    try {
      const response = await fetch('/todos', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ title: newTodoTitle }),
      })
      if (response.ok) {
        setNewTodoTitle('')
        fetchTodos()
      } else {
        alert('添加失败')
      }
    } catch (error) {
      console.error('Error adding todo:', error)
    }
  }

  const handleLogout = () => {
    setToken(null)
    localStorage.removeItem('token')
    setTodos([])
  }

  if (!token) {
    return (
      <div className="App">
        <h1>登录</h1>
        <form onSubmit={handleLogin}>
          <div style={{ marginBottom: '10px' }}>
            <label style={{ marginRight: '10px' }}>邮箱:</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>
          <div style={{ marginBottom: '10px' }}>
            <label style={{ marginRight: '10px' }}>密码:</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>
          <button type="submit">登录</button>
        </form>
      </div>
    )
  }

  return (
    <div className="App">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h1>Todo 列表</h1>
        <button onClick={handleLogout}>退出登录</button>
      </div>

      <form onSubmit={handleAddTodo} style={{ marginBottom: '20px' }}>
        <input
          type="text"
          placeholder="输入新的 Todo..."
          value={newTodoTitle}
          onChange={(e) => setNewTodoTitle(e.target.value)}
          style={{ padding: '8px', width: '250px', marginRight: '10px' }}
        />
        <button type="submit">添加</button>
      </form>

      <ul style={{ listStyle: 'none', padding: 0 }}>
        {todos.map((todo) => (
          <li
            key={todo.id}
            style={{
              padding: '10px',
              borderBottom: '1px solid #eee',
              textAlign: 'left',
              display: 'flex',
              alignItems: 'center'
            }}
          >
            <input type="checkbox" checked={todo.completed} readOnly style={{ marginRight: '10px' }} />
            <span style={{ textDecoration: todo.completed ? 'line-through' : 'none' }}>
              {todo.title}
            </span>
          </li>
        ))}
      </ul>
      {todos.length === 0 && <p>暂无待办事项</p>}
    </div>
  )
}

export default App
