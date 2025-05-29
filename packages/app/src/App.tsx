import {createIdBySeed} from '@internal/utils'

function App() {
  return <div className='grid m-6 place-content-center'>
    <h1 className="text-3xl">Hello</h1>
    <p>{createIdBySeed("Cowboy")}</p>
  </div>
}

export default App
