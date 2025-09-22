import { Verre } from './index.js'

const app = new Verre()

app.get('/', () => ({
  status: 200,
  body: 'Hello from Verre!'
}))

app.get('/json', async (_err, req) => {
  const json: unknown = await req.json()

  return {
    status: 200,
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(json)
  }
})

app.serve()
