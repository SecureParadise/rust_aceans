import './App.css'
import { useEffect, useState } from 'react'
import axios from 'axios'

function App() {
  const [jokes,setJokes] = useState([])

  useEffect(()=>{
    axios.get('/api/jokes')
    .then((response) =>
    {
      setJokes(response.data)
    })
    .catch((error) =>{
      console.log(error);
      
    })
  })


  return (
    <>
    <h1>Hello Joke Len: {jokes.length}</h1>
     {
      jokes.map((joke,index)=>{
       if (joke.type =='twopart'){
        return <>
        <h2>{joke.setup}</h2>
        <h2>{joke.delivery}</h2>
        </>
      }
      else if (joke.type =='single'){
        return <h2>{joke.joke}</h2>
      }
    }
    )
     }
    </>
  )
}

export default App
