import { Verre } from './index.js'

const app = new Verre()

app.get('/', () => ({
  status: 200,
  body: 'Hello from Verre!'
}))

app.serve()
