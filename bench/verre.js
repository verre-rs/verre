import { Verre } from '../index.js'

const app = new Verre()

app.get('/', () => ({
  status: 200,
  headers: {
    'Content-Type': 'text/plain',
  },
  body: 'Hello from Verre!'
}))

app.get('/id/{id}', (_err, req) => {
  const id = new URL(req.url, 'http://example.com').pathname.slice(4)
  const name = new URLSearchParams(req.query ?? '').get('name')

  return {
    status: 200,
    headers: {
      'Content-Type': 'text/plain',
    },
    body: `${id} ${name}`
  }
})

app.get('/json', async (_err, req) => {
  const json = await req.json()

  return {
    status: 200,
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(json)
  }
})

app.serve()
console.log('Listening to port 3000')
